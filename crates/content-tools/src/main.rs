use rebrng_game_core::{CommandError, ContentBundle, ContentSource};
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
    let manifest_path = input.join("manifest.yaml");
    let source_text = fs::read_to_string(&manifest_path)?;
    let source: ContentSource = serde_yaml::from_str(&source_text)?;
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
