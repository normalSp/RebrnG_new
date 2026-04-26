use rebrng_game_core::{CommandError, ContentBundle, ContentSource, ContentSourceFragment};
use std::env;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;

const USAGE: &str =
    "Usage: rebrng-content-tools build-s0 --input content/s0 --output target/rebrng-content/s0.bundle.json";

#[derive(Debug)]
enum ToolError {
    Usage(String),
    Content(CommandError),
    Io(std::io::Error),
    Yaml(serde_yaml::Error),
    Json(serde_json::Error),
}

impl ToolError {
    fn exit_code(&self) -> i32 {
        match self {
            Self::Usage(_) => 2,
            Self::Content(_) => 3,
            Self::Io(_) | Self::Yaml(_) | Self::Json(_) => 1,
        }
    }
}

impl fmt::Display for ToolError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Usage(message) => write!(formatter, "{message}\n{USAGE}"),
            Self::Content(error) => write!(
                formatter,
                "{}: {}",
                error.message,
                error.diagnostics.clone().unwrap_or_default()
            ),
            Self::Io(error) => write!(formatter, "I/O error: {error}"),
            Self::Yaml(error) => write!(formatter, "YAML error: {error}"),
            Self::Json(error) => write!(formatter, "JSON error: {error}"),
        }
    }
}

impl From<std::io::Error> for ToolError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<serde_yaml::Error> for ToolError {
    fn from(value: serde_yaml::Error) -> Self {
        Self::Yaml(value)
    }
}

impl From<serde_json::Error> for ToolError {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value)
    }
}

impl From<CommandError> for ToolError {
    fn from(value: CommandError) -> Self {
        Self::Content(value)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct BuildArgs {
    input: PathBuf,
    output: PathBuf,
}

fn main() {
    if let Err(error) = run(env::args().skip(1)) {
        eprintln!("{error}");
        process::exit(error.exit_code());
    }
}

fn run(args: impl IntoIterator<Item = String>) -> Result<(), ToolError> {
    let args = parse_args(args)?;
    build_s0(&args.input, &args.output)
}

fn parse_args(args: impl IntoIterator<Item = String>) -> Result<BuildArgs, ToolError> {
    let mut args = args.into_iter();

    match args.next().as_deref() {
        Some("build-s0") => {}
        Some(other) => return Err(ToolError::Usage(format!("unknown command '{other}'"))),
        None => return Err(ToolError::Usage("missing command".to_string())),
    }

    let mut input = None;
    let mut output = None;

    while let Some(flag) = args.next() {
        match flag.as_str() {
            "--input" => input = args.next().map(PathBuf::from),
            "--output" => output = args.next().map(PathBuf::from),
            other => return Err(ToolError::Usage(format!("unknown option '{other}'"))),
        }
    }

    Ok(BuildArgs {
        input: input.ok_or_else(|| ToolError::Usage("missing --input".to_string()))?,
        output: output.ok_or_else(|| ToolError::Usage("missing --output".to_string()))?,
    })
}

fn build_s0(input: &Path, output: &Path) -> Result<(), ToolError> {
    let source = load_s0_source(input)?;
    let bundle = ContentBundle::from_source(source)?;
    let bundle_json = serde_json::to_string_pretty(&bundle)?;

    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(output, bundle_json)?;
    println!(
        "built {} v{} -> {}",
        bundle.manifest.content_id,
        bundle.manifest.version,
        output.display()
    );
    Ok(())
}

fn load_s0_source(input: &Path) -> Result<ContentSource, ToolError> {
    let mut yaml_files = Vec::new();
    collect_yaml_files(input, &mut yaml_files)?;
    yaml_files.sort();

    if yaml_files.is_empty() {
        return Err(ToolError::Usage(format!(
            "no YAML content files found under {}",
            input.display()
        )));
    }

    let mut fragments = Vec::with_capacity(yaml_files.len());
    for yaml_file in yaml_files {
        let source_text = fs::read_to_string(&yaml_file)?;
        let fragment: ContentSourceFragment = serde_yaml::from_str(&source_text)?;
        fragments.push(fragment);
    }

    Ok(ContentSource::from_fragments(fragments)?)
}

fn collect_yaml_files(input: &Path, yaml_files: &mut Vec<PathBuf>) -> Result<(), ToolError> {
    for entry in fs::read_dir(input)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_yaml_files(&path, yaml_files)?;
            continue;
        }

        if path
            .extension()
            .and_then(|extension| extension.to_str())
            .is_some_and(|extension| matches!(extension, "yaml" | "yml"))
        {
            yaml_files.push(path);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn parse_build_s0_args() {
        let args = parse_args([
            "build-s0".to_string(),
            "--input".to_string(),
            "content/s0".to_string(),
            "--output".to_string(),
            "target/rebrng-content/s0.bundle.json".to_string(),
        ])
        .expect("args should parse");

        assert_eq!(
            args,
            BuildArgs {
                input: PathBuf::from("content/s0"),
                output: PathBuf::from("target/rebrng-content/s0.bundle.json"),
            }
        );
    }

    #[test]
    fn load_s0_source_reads_split_yaml_fragments() {
        let root = unique_temp_dir();
        fs::create_dir_all(root.join("nodes")).expect("nodes dir");
        fs::create_dir_all(root.join("actions")).expect("actions dir");
        fs::create_dir_all(root.join("routes")).expect("routes dir");
        fs::create_dir_all(root.join("windows")).expect("windows dir");

        fs::write(
            root.join("manifest.yaml"),
            r#"
content_id: s0.qingmao.foundation
version: s0.0.1
title: 青茅山 Sprint 0 内容骨架
stage: s0
entry_scene_id: academy_gate
"#,
        )
        .expect("write manifest");
        fs::write(
            root.join("nodes/academy.yaml"),
            r#"
nodes:
  - id: academy_gate
    title: 学堂门前
    safety: low
    stage: s0
    tags: [node, academy]
    evidence: canon_inferred
    modes: [canon_strict, sandbox_if]
"#,
        )
        .expect("write nodes");
        fs::write(
            root.join("actions/core.yaml"),
            r#"
actions:
  - id: scout_academy
    label: 观察学堂风声
    intent: scout
    target: academy_gate
    stage: s0
    tags: [action, scout]
    evidence: canon_inferred
    modes: [canon_strict, sandbox_if]
    importance: standard
"#,
        )
        .expect("write actions");
        fs::write(
            root.join("routes/routes.yaml"),
            r#"
routes:
  - id: moonlight_entry
    label: 月光修行入口
    route: moonlight
    entry_action_ids: [scout_academy]
    stage: s0
    tags: [route, moonlight]
    evidence: canon_inferred
    modes: [canon_strict, sandbox_if]
"#,
        )
        .expect("write routes");
        fs::write(
            root.join("windows/windows.yaml"),
            r#"
windows:
  - id: day1_morning_free
    day: 1
    period: 清晨
    window_type: free
    default_ap: 2
    stage: s0
    tags: [window, opening]
    evidence: canon_inferred
    modes: [canon_strict, sandbox_if]
"#,
        )
        .expect("write windows");

        let source = load_s0_source(&root).expect("split source should load");

        assert_eq!(source.nodes.len(), 1);
        assert_eq!(source.actions.len(), 1);
        assert_eq!(source.routes.len(), 1);
        assert_eq!(source.windows.len(), 1);
        assert_eq!(source.entry_scene_id, "academy_gate");

        fs::remove_dir_all(root).expect("cleanup temp dir");
    }

    fn unique_temp_dir() -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time")
            .as_nanos();
        std::env::temp_dir().join(format!("rebrng-content-tools-test-{nanos}"))
    }
}
