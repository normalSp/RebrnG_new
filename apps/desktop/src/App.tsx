import { invoke } from "@tauri-apps/api/core";
import {
  LedgerShell,
  type ActionCommand,
  type ActionResponse,
  type CommandError,
  type LedgerViewModel,
  type SaveWriteResult,
} from "@rebrng/ui-ledger";
import { useState } from "react";
import "./App.css";

function App() {
  const [projection, setProjection] = useState<LedgerViewModel | null>(null);
  const [status, setStatus] = useState("等待创建 active run");

  async function createRun() {
    setStatus("正在请求 Rust 创建单局...");
    try {
      const response = await invoke<ActionResponse>("create_run", {
        mode: "canon_strict",
      });
      setProjection(response.projection);
      setStatus(
        `新局已建立：${currentNodeTitle(response.projection)}。${recentSummary(response.projection)}`,
      );
    } catch (error) {
      setStatus(formatCommandError(error));
    }
  }

  async function resolveAction(command: ActionCommand) {
    const label = command.context_note ?? command.intent;
    setStatus(`正在结算：${label}，账本正在回算...`);
    try {
      const response = await invoke<ActionResponse>("resolve_action", {
        command,
      });
      setProjection(response.projection);
      setStatus(describeResolvedAction(label, command, response));
    } catch (error) {
      setStatus(formatCommandError(error));
    }
  }

  async function writeSave() {
    setStatus("正在写入 slot_0...");
    try {
      const result = await invoke<SaveWriteResult>("write_save", {
        slotId: "slot_0",
      });
      setStatus(
        `已写入：${result.path_hint} / 检查点 ${result.checkpoint_count} 个 / 当前快照 ${result.current_checkpoint_id}`,
      );
    } catch (error) {
      setStatus(formatCommandError(error));
    }
  }

  async function loadSave() {
    setStatus("正在读取 slot_0...");
    try {
      const response = await invoke<ActionResponse>("load_save", {
        slotId: "slot_0",
      });
      setProjection(response.projection);
      setStatus(
        `已读回：${currentNodeTitle(response.projection)} / save_load ${response.performance.save_load_ms}ms`,
      );
    } catch (error) {
      setStatus(formatCommandError(error));
    }
  }

  return (
    <LedgerShell
      projection={projection}
      status={status}
      onCreateRun={createRun}
      onResolveAction={resolveAction}
      onWriteSave={writeSave}
      onLoadSave={loadSave}
    />
  );
}

function describeResolvedAction(
  label: string,
  command: ActionCommand,
  response: ActionResponse,
): string {
  const projection = response.projection;
  const location = currentNodeTitle(projection);
  const feedback = recentSummary(projection);
  const elapsed = response.performance.resolve_action_ms;

  const prefix = (() => {
    switch (command.intent) {
      case "move":
        return `已移动到：${location}`;
      case "cultivate":
        return "已修行落账";
      case "scout":
        return "已记录风声线索";
      case "recover":
        return "已恢复并记下债务";
      case "trade":
        return "已完成交易并抬高暴露";
      case "retreat":
        return "已脱离遭遇";
      case "confront":
        return "已硬顶遭遇，代价已落账";
      case "yield":
      case "argue":
      case "delay":
      case "frame":
        return "已处理遭遇决断";
      case "wait":
        return `已推进窗口：${projection.current_period}`;
      default:
        return `已结算：${label}`;
    }
  })();

  return `${prefix}。${feedback} / resolve_action ${elapsed}ms`;
}

function currentNodeTitle(projection: LedgerViewModel): string {
  return (
    projection.node_view.visible_nodes.find((node) => node.current)?.title ??
    projection.current_node_id
  );
}

function recentSummary(projection: LedgerViewModel): string {
  return projection.recent_feedback?.summary ?? projection.scene_text;
}

function formatCommandError(error: unknown): string {
  if (isCommandError(error)) {
    return `${error.kind}: ${error.message}`;
  }

  return `internal: ${String(error)}`;
}

function isCommandError(error: unknown): error is CommandError {
  return (
    typeof error === "object" &&
    error !== null &&
    "kind" in error &&
    "message" in error
  );
}

export default App;
