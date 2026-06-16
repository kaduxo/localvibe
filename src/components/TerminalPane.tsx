import { useRef } from "react";
import "@xterm/xterm/css/xterm.css";
import { usePty } from "../hooks/usePty";
import type { ResolvedPane } from "../types/template";

interface TerminalPaneProps {
  pane: ResolvedPane;
  active?: boolean;
}

export function TerminalPane({ pane, active = true }: TerminalPaneProps) {
  const containerRef = useRef<HTMLDivElement>(null);
  const { exitCode } = usePty(pane, containerRef, active);

  return (
    <div className="terminal-pane">
      <div className="terminal-pane-header">
        <span className="terminal-pane-title">{pane.title}</span>
        <span className="terminal-pane-status">
          {pane.resolvedCwd}
        </span>
        {exitCode !== null && (
          <span className="terminal-pane-status exited">exit {exitCode}</span>
        )}
      </div>
      <div className="terminal-pane-body" ref={containerRef} />
    </div>
  );
}