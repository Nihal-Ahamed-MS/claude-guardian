import type { Session } from "../App";

type Props = {
  sessions: Session[];
  selectedId: string | null;
  onSelect: (id: string) => void;
};

const fmt = (ts: number) =>
  new Date(ts).toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" });

export default function SessionList({ sessions, selectedId, onSelect }: Props) {
  return (
    <div>
      <div style={{ fontSize: 11, color: "#666", marginBottom: 8, textTransform: "uppercase", letterSpacing: "0.08em" }}>
        Sessions
      </div>
      {sessions.length === 0 && (
        <div style={{ fontSize: 12, color: "#444" }}>No sessions yet</div>
      )}
      {sessions.map((s) => (
        <button
          key={s.id}
          onClick={() => onSelect(s.id)}
          style={{
            display: "block",
            width: "100%",
            textAlign: "left",
            background: selectedId === s.id ? "#1a1a2e" : "transparent",
            border: selectedId === s.id ? "1px solid #334" : "1px solid transparent",
            borderRadius: 6,
            padding: "8px 10px",
            cursor: "pointer",
            marginBottom: 4,
            color: "#e0e0e0",
            fontFamily: "inherit",
          }}
        >
          <div style={{ fontSize: 11, color: "#888", marginBottom: 2 }}>{fmt(s.started_at)}</div>
          <div style={{ fontSize: 12, wordBreak: "break-all" }}>{s.id.slice(0, 20)}…</div>
          <div style={{ fontSize: 11, color: "#4ade80", marginTop: 2 }}>{s.event_count} events</div>
        </button>
      ))}
    </div>
  );
}
