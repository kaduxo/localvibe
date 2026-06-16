import { useEffect, useRef, useState } from "react";
import { FitAddon } from "@xterm/addon-fit";
import { Terminal } from "@xterm/xterm";
import { spawn, type IPty } from "tauri-pty";
import type { ResolvedPane } from "../types/template";

function decodeOutput(data: Uint8Array): string {
  return new TextDecoder("utf-8").decode(data);
}

export function usePty(
  pane: ResolvedPane,
  containerRef: React.RefObject<HTMLDivElement | null>,
  active: boolean
) {
  const fitRef = useRef<FitAddon | null>(null);
  const [exitCode, setExitCode] = useState<number | null>(null);

  useEffect(() => {
    const container = containerRef.current;
    if (!container || !active) return;

    const term = new Terminal({
      convertEol: true,
      cursorBlink: true,
      fontFamily: "Cascadia Mono, Consolas, monospace",
      fontSize: 13,
      theme: {
        background: "#010409",
        foreground: "#e6edf3",
        cursor: "#58a6ff",
      },
      scrollback: 1000,
    });

    const fitAddon = new FitAddon();
    term.loadAddon(fitAddon);
    term.open(container);
    fitAddon.fit();
    fitRef.current = fitAddon;

    const pty: IPty = spawn(pane.shellPath, [], {
      cols: term.cols,
      rows: term.rows,
      cwd: pane.resolvedCwd,
      env: pane.resolvedEnv,
    });

    pty.onData((data) => {
      term.write(decodeOutput(data));
    });

    pty.onExit(({ exitCode: code }) => {
      setExitCode(code);
      term.writeln(`\r\n\x1b[90m[process exited: ${code}]\x1b[0m`);
    });

    term.onData((data) => {
      pty.write(data);
    });

    term.onResize(({ cols, rows }) => {
      pty.resize(cols, rows);
    });

    let commandTimer: number | undefined;
    if (pane.resolvedCommand.trim()) {
      commandTimer = window.setTimeout(() => {
        pty.write(`${pane.resolvedCommand}\r`);
      }, 400);
    }

    const observer = new ResizeObserver(() => {
      fitAddon.fit();
    });
    observer.observe(container);

    const onWindowResize = () => fitAddon.fit();
    window.addEventListener("resize", onWindowResize);

    return () => {
      if (commandTimer) window.clearTimeout(commandTimer);
      observer.disconnect();
      window.removeEventListener("resize", onWindowResize);
      pty.kill();
      term.dispose();
      fitRef.current = null;
    };
  }, [
    active,
    containerRef,
    pane.id,
    pane.resolvedCommand,
    pane.resolvedCwd,
    pane.resolvedEnv,
    pane.shellPath,
  ]);

  return { exitCode };
}