import { Fragment } from "react";
import {
  Panel,
  PanelGroup,
  PanelResizeHandle,
} from "react-resizable-panels";
import type { ResolvedPane, LayoutConfig } from "../types/template";
import { TerminalPane } from "./TerminalPane";

interface WorkspaceLayoutProps {
  layout: LayoutConfig;
  panes: ResolvedPane[];
}

function buildRows(panes: ResolvedPane[], rows: number, cols: number) {
  const grid: ResolvedPane[][] = [];
  for (let r = 0; r < rows; r++) {
    const row: ResolvedPane[] = [];
    for (let c = 0; c < cols; c++) {
      const index = r * cols + c;
      if (panes[index]) row.push(panes[index]);
    }
    if (row.length > 0) grid.push(row);
  }
  return grid;
}

function HorizontalRow({ panes }: { panes: ResolvedPane[] }) {
  return (
    <PanelGroup direction="horizontal">
      {panes.map((pane, index) => (
        <Fragment key={pane.id}>
          {index > 0 && <PanelResizeHandle className="resize-handle" />}
          <Panel defaultSize={100 / panes.length} minSize={15}>
            <TerminalPane pane={pane} />
          </Panel>
        </Fragment>
      ))}
    </PanelGroup>
  );
}

export function WorkspaceLayout({ layout, panes }: WorkspaceLayoutProps) {
  const rows = buildRows(panes, layout.rows, layout.cols);

  if (layout.rows === 1) {
    return (
      <div className="workspace-grid">
        <HorizontalRow panes={panes} />
      </div>
    );
  }

  return (
    <PanelGroup direction="vertical" className="workspace-grid">
      {rows.map((row, rowIndex) => (
        <Fragment key={`row-${rowIndex}`}>
          {rowIndex > 0 && <PanelResizeHandle className="resize-handle" />}
          <Panel defaultSize={100 / rows.length} minSize={15}>
            <HorizontalRow panes={row} />
          </Panel>
        </Fragment>
      ))}
    </PanelGroup>
  );
}