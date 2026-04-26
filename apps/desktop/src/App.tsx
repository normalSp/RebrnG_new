import { invoke } from "@tauri-apps/api/core";
import {
  LedgerShell,
  type ActionCommand,
  type ActionResponse,
  type CommandError,
  type LedgerViewModel,
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
      setStatus("active run 已由 Rust 托管");
    } catch (error) {
      setStatus(formatCommandError(error));
    }
  }

  async function resolveAction(command: ActionCommand) {
    setStatus(`正在结算 ${command.intent}...`);
    try {
      const response = await invoke<ActionResponse>("resolve_action", {
        command,
      });
      setProjection(response.projection);
      setStatus(
        `已结算：resolve_action ${response.performance.resolve_action_ms}ms`,
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
    />
  );
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
