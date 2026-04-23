import { useEffect, useState } from "react";
import type { HookEvent } from "../App";

type Props = {
  sessionId: string | null;
  liveEvents: HookEvent[];
};

const EVENT_COLORS: Record<string, string> = {
  PreToolUse: "#60a5fa",
  PostToolUse: "#34d399",
  Stop: "#f87171",
  Notification: "#fbbf24",
  unknown: "#9ca3af",
};

const fmt = (ts: number) =>
  new Date(ts).toLocaleTimeString([], { hour: "2-digit", minute: "2-digit", second: "2-digit" });

export default function EventFeed({ sessionId, liveEvents }: Props) {
  const [storedEvents, setStoredEvents] = useState<HookEvent[]>([]);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    if (!sessionId) return;
    setLoading(true);
    fetch(`/api/sessions/${sessionId}/events?limit=200`)
      .then((r) => r.json())
      .then((data: HookEvent[]) => setStoredEvents(data))
      .catch(console.error)
      .finally(() => setLoading(false));
  }, [sessionId]);

  const events = sessionId
    ? storedEvents
    : liveEvents;

  return (
    <div style={{ flex: 1, overflowY: "auto", padding: 16 }}>
      {!sessionId && (
        <div style={{ color: "#666", fontSize: 12, marginBottom: 12 }}>
          Showing live stream — select a session to view history
        </div>
      )}
      {loading && <div style={{ color: "#666", fontSize: 12 }}>Loading…</div>}
      {events.map((ev) => (
        <EventRow key={ev.id} event={ev} />
      ))}
      {events.length === 0 && !loading && (
        <div style={{ color: "#444", fontSize: 12 }}>No events</div>
      )}
    </div>
  );
}

function EventRow({ event }: { event: HookEvent }) {
  const [expanded, setExpanded] = useState(false);
  const color = EVENT_COLORS[event.event_type] ?? EVENT_COLORS.unknown;

  return (
    <div
      style={{
        borderLeft: `2px solid ${color}`,
        paddingLeft: 12,
        marginBottom: 10,
        cursor: "pointer",
      }}
      onClick={() => setExpanded((e) => !e)}
    >
      <div style={{ display: "flex", gap: 10, alignItems: "baseline" }}>
        <span style={{ color, fontSize: 11, fontWeight: 600 }}>{event.event_type}</span>
        <span style={{ color: "#555", fontSize: 11 }}>{fmt(event.ts)}</span>
      </div>
      {expanded && (
        <pre
          style={{
            marginTop: 6,
            fontSize: 11,
            color: "#b0b0b0",
            whiteSpace: "pre-wrap",
            wordBreak: "break-all",
            background: "#111",
            borderRadius: 4,
            padding: 8,
          }}
        >
          {JSON.stringify(event.payload, null, 2)}
        </pre>
      )}
    </div>
  );
}
