import {
  useEffect,
  useMemo,
  useState,
  type CSSProperties,
  type ReactNode,
} from "react";
import { fetchAlerts, fetchLogs } from "../services/api";
import SeverityBadge, { normalizeSeverity } from "../components/SeverityBadge";

const SEVERITY_COLORS: Record<string, string> = {
  critical: "#ef4444",
  high: "#f97316",
  medium: "#eab308",
  low: "#3b82f6",
  info: "#64748b",
};

function StatCard({ label, value, color }: { label: string; value: number; color: string }) {
  return (
    <div className="stat-card" style={{ "--stat-color": color } as CSSProperties}>
      <div className="stat-label">{label}</div>
      <div className="stat-value">{value}</div>
    </div>
  );
}

/* Area chart: volume de logs par heure (24 dernières heures) */
function ActivityChart({ logs }: { logs: any[] }) {
  const buckets = useMemo(() => {
    const now = new Date();
    const counts = new Array(24).fill(0);
    for (const log of logs) {
      const t = new Date(log.created_at);
      const hoursAgo = Math.floor((now.getTime() - t.getTime()) / 3_600_000);
      if (hoursAgo >= 0 && hoursAgo < 24) {
        counts[23 - hoursAgo] += 1;
      }
    }
    return counts;
  }, [logs]);

  const width = 600;
  const height = 180;
  const max = Math.max(...buckets, 1);
  const stepX = width / (buckets.length - 1);

  const points = buckets.map((count, i) => ({
    x: i * stepX,
    y: height - (count / max) * (height - 20) - 4,
  }));

  const line = points.map((p) => `${p.x},${p.y}`).join(" ");
  const area = `0,${height} ${line} ${width},${height}`;

  return (
    <svg viewBox={`0 0 ${width} ${height + 20}`} style={{ width: "100%", height: "auto" }}>
      <defs>
        <linearGradient id="areaFill" x1="0" y1="0" x2="0" y2="1">
          <stop offset="0%" stopColor="#8b5cf6" stopOpacity="0.45" />
          <stop offset="100%" stopColor="#8b5cf6" stopOpacity="0.02" />
        </linearGradient>
      </defs>
      {[0.25, 0.5, 0.75].map((r) => (
        <line
          key={r}
          x1="0"
          x2={width}
          y1={height * r}
          y2={height * r}
          stroke="#1e293b"
          strokeDasharray="4 6"
        />
      ))}
      <polygon points={area} fill="url(#areaFill)" />
      <polyline points={line} fill="none" stroke="#a78bfa" strokeWidth="2.5" strokeLinejoin="round" />
      {points.map((p, i) =>
        buckets[i] > 0 ? <circle key={i} cx={p.x} cy={p.y} r="3" fill="#ddd6fe" /> : null
      )}
      <text x="0" y={height + 16} fill="#64748b" fontSize="11">
        -24h
      </text>
      <text x={width} y={height + 16} fill="#64748b" fontSize="11" textAnchor="end">
        maintenant
      </text>
    </svg>
  );
}

/* Donut: répartition des logs par sévérité */
function SeverityDonut({ logs }: { logs: any[] }) {
  const counts = useMemo(() => {
    const acc: Record<string, number> = {};
    for (const log of logs) {
      const level = normalizeSeverity(log.severity);
      acc[level] = (acc[level] ?? 0) + 1;
    }
    return acc;
  }, [logs]);

  const total = logs.length;
  const order = ["critical", "high", "medium", "low", "info"].filter((s) => counts[s]);

  const radius = 60;
  const stroke = 22;
  const circumference = 2 * Math.PI * radius;
  let offset = 0;

  return (
    <div style={{ display: "flex", alignItems: "center", gap: 24, flexWrap: "wrap" }}>
      <svg width="160" height="160" viewBox="0 0 160 160">
        <circle cx="80" cy="80" r={radius} fill="none" stroke="#1e293b" strokeWidth={stroke} />
        {order.map((level) => {
          const fraction = counts[level] / total;
          const dash = fraction * circumference;
          const el = (
            <circle
              key={level}
              cx="80"
              cy="80"
              r={radius}
              fill="none"
              stroke={SEVERITY_COLORS[level]}
              strokeWidth={stroke}
              strokeDasharray={`${dash} ${circumference - dash}`}
              strokeDashoffset={-offset}
              transform="rotate(-90 80 80)"
            />
          );
          offset += dash;
          return el;
        })}
        <text x="80" y="76" textAnchor="middle" fill="#e2e8f0" fontSize="26" fontWeight="700">
          {total}
        </text>
        <text x="80" y="96" textAnchor="middle" fill="#64748b" fontSize="11">
          événements
        </text>
      </svg>

      <div className="chart-legend">
        {order.map((level) => (
          <div className="legend-row" key={level}>
            <span className="legend-swatch" style={{ background: SEVERITY_COLORS[level] }} />
            {level}
            <span className="legend-count">{counts[level]}</span>
          </div>
        ))}
        {total === 0 && <div className="legend-row">Aucun événement</div>}
      </div>
    </div>
  );
}

/* Bar list: sources les plus actives */
function TopSources({ logs }: { logs: any[] }) {
  const top = useMemo(() => {
    const acc: Record<string, number> = {};
    for (const log of logs) {
      const s = log.source_name ?? "—";
      acc[s] = (acc[s] ?? 0) + 1;
    }
    return Object.entries(acc)
      .sort((a, b) => b[1] - a[1])
      .slice(0, 6);
  }, [logs]);

  const max = Math.max(...top.map(([, n]) => n), 1);

  return (
    <div className="panel">
      <h3 className="panel-title">Top sources</h3>
      <div className="bar-list">
        {top.map(([name, n]) => (
          <div className="bar-row" key={name}>
            <span className="bar-label cell-mono">{name}</span>
            <span className="bar-track">
              <span className="bar-fill" style={{ width: `${(n / max) * 100}%` }} />
            </span>
            <span className="bar-value">{n}</span>
          </div>
        ))}
        {top.length === 0 && <div className="empty-state">Aucune source</div>}
      </div>
    </div>
  );
}

/* Table: dernières alertes */
function RecentAlertsTable({ alerts }: { alerts: any[] }) {
  const recent = alerts.slice(0, 5);
  return (
    <div className="table-wrap">
      <table className="data-table">
        <thead>
          <tr>
            <th>Dernières alertes</th>
            <th>Sévérité</th>
            <th>Source</th>
            <th>Description</th>
            <th>Horodatage</th>
          </tr>
        </thead>
        <tbody>
          {recent.map((alert) => (
            <tr key={alert.id}>
              <td className="cell-strong">{alert.rule_name}</td>
              <td>
                <SeverityBadge severity={alert.severity} />
              </td>
              <td className="cell-mono">{alert.source_name ?? "—"}</td>
              <td>{alert.description}</td>
              <td className="cell-mono">{new Date(alert.timestamp).toLocaleString()}</td>
            </tr>
          ))}
          {recent.length === 0 && (
            <tr>
              <td colSpan={5}>
                <div className="empty-state">Aucune alerte — système sain</div>
              </td>
            </tr>
          )}
        </tbody>
      </table>
    </div>
  );
}

/* Table: derniers logs */
function RecentLogsTable({ logs }: { logs: any[] }) {
  const recent = logs.slice(0, 8);
  return (
    <div className="table-wrap">
      <table className="data-table">
        <thead>
          <tr>
            <th>Derniers logs</th>
            <th>Type d'événement</th>
            <th>Sévérité</th>
            <th>Message</th>
            <th>Horodatage</th>
          </tr>
        </thead>
        <tbody>
          {recent.map((log) => (
            <tr key={log.id}>
              <td className="cell-strong cell-mono">{log.source_name}</td>
              <td className="cell-mono">{log.event_type}</td>
              <td>
                <SeverityBadge severity={log.severity} />
              </td>
              <td>{log.message}</td>
              <td className="cell-mono">{new Date(log.created_at).toLocaleString()}</td>
            </tr>
          ))}
          {recent.length === 0 && (
            <tr>
              <td colSpan={5}>
                <div className="empty-state">Aucun log collecté</div>
              </td>
            </tr>
          )}
        </tbody>
      </table>
    </div>
  );
}

/* ---------- Registre des widgets ---------- */

type WidgetCtx = { logs: any[]; alerts: any[] };

type WidgetDef = {
  id: string;
  label: string;
  span: number; // 1 à 4 colonnes
  render: (ctx: WidgetCtx) => ReactNode;
};

const highSeverity = (alerts: any[]) =>
  alerts.filter((a) => ["critical", "high"].includes(normalizeSeverity(a.severity))).length;

const WIDGETS: WidgetDef[] = [
  {
    id: "stat-events",
    label: "Événements collectés",
    span: 1,
    render: ({ logs }) => <StatCard label="Événements collectés" value={logs.length} color="#8b5cf6" />,
  },
  {
    id: "stat-alerts",
    label: "Alertes actives",
    span: 1,
    render: ({ alerts }) => <StatCard label="Alertes actives" value={alerts.length} color="#eab308" />,
  },
  {
    id: "stat-critical",
    label: "Alertes critiques / high",
    span: 1,
    render: ({ alerts }) => (
      <StatCard label="Alertes critiques / high" value={highSeverity(alerts)} color="#ef4444" />
    ),
  },
  {
    id: "stat-sources",
    label: "Sources surveillées",
    span: 1,
    render: ({ logs }) => (
      <StatCard
        label="Sources surveillées"
        value={new Set(logs.map((l) => l.source_name)).size}
        color="#22d3ee"
      />
    ),
  },
  {
    id: "chart-activity",
    label: "Activité des logs (24h)",
    span: 2,
    render: ({ logs }) => (
      <div className="panel">
        <h3 className="panel-title">Activité des logs — 24 dernières heures</h3>
        <ActivityChart logs={logs} />
      </div>
    ),
  },
  {
    id: "chart-severity",
    label: "Répartition par sévérité",
    span: 2,
    render: ({ logs }) => (
      <div className="panel">
        <h3 className="panel-title">Répartition par sévérité</h3>
        <SeverityDonut logs={logs} />
      </div>
    ),
  },
  {
    id: "top-sources",
    label: "Top sources",
    span: 2,
    render: ({ logs }) => <TopSources logs={logs} />,
  },
  {
    id: "table-alerts",
    label: "Dernières alertes",
    span: 4,
    render: ({ alerts }) => <RecentAlertsTable alerts={alerts} />,
  },
  {
    id: "table-logs",
    label: "Derniers logs",
    span: 4,
    render: ({ logs }) => <RecentLogsTable logs={logs} />,
  },
];

const DEFAULT_LAYOUT = [
  "stat-events",
  "stat-alerts",
  "stat-critical",
  "stat-sources",
  "chart-activity",
  "chart-severity",
  "table-alerts",
];

const STORAGE_KEY = "medtech-dashboard-layout-v1";

function useDashboardLayout() {
  const [layout, setLayout] = useState<string[]>(() => {
    try {
      const saved = localStorage.getItem(STORAGE_KEY);
      if (saved) {
        const parsed = JSON.parse(saved);
        if (Array.isArray(parsed)) {
          const valid = parsed.filter((id) => WIDGETS.some((w) => w.id === id));
          if (valid.length > 0) return valid;
        }
      }
    } catch {
      /* config corrompue -> on retombe sur le layout par défaut */
    }
    return DEFAULT_LAYOUT;
  });

  useEffect(() => {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(layout));
  }, [layout]);

  return [layout, setLayout] as const;
}

function DashboardPage() {
  const [logs, setLogs] = useState<any[]>([]);
  const [alerts, setAlerts] = useState<any[]>([]);
  const [editing, setEditing] = useState(false);
  const [layout, setLayout] = useDashboardLayout();

  useEffect(() => {
    const load = () => {
      fetchLogs().then((data) => setLogs(Array.isArray(data) ? data : []));
      fetchAlerts().then((data) => setAlerts(Array.isArray(data) ? data : []));
    };
    load();
    const interval = setInterval(load, 10_000);
    return () => clearInterval(interval);
  }, []);

  const ctx: WidgetCtx = { logs, alerts };
  const available = WIDGETS.filter((w) => !layout.includes(w.id));

  const moveWidget = (index: number, dir: -1 | 1) => {
    setLayout((current) => {
      const next = [...current];
      const target = index + dir;
      if (target < 0 || target >= next.length) return current;
      [next[index], next[target]] = [next[target], next[index]];
      return next;
    });
  };

  const removeWidget = (id: string) => setLayout((current) => current.filter((x) => x !== id));
  const addWidget = (id: string) => setLayout((current) => [...current, id]);

  return (
    <div>
      <div className="dash-toolbar">
        {editing && (
          <button className="btn btn-ghost" style={{ width: "auto" }} onClick={() => setLayout(DEFAULT_LAYOUT)}>
            Réinitialiser
          </button>
        )}
        <button
          className={editing ? "btn btn-primary" : "btn"}
          onClick={() => setEditing((e) => !e)}
        >
          {editing ? "✓ Terminer" : "✎ Personnaliser"}
        </button>
      </div>

      {editing && (
        <div className="widget-catalog">
          <div className="catalog-title">Ajouter un widget</div>
          {available.length === 0 ? (
            <div style={{ color: "var(--text-faint)", fontSize: 13 }}>
              Tous les widgets sont déjà affichés.
            </div>
          ) : (
            <div className="catalog-chips">
              {available.map((w) => (
                <button key={w.id} className="catalog-chip" onClick={() => addWidget(w.id)}>
                  <span className="plus">+</span>
                  {w.label}
                </button>
              ))}
            </div>
          )}
        </div>
      )}

      <div className="widget-grid">
        {layout.map((id, i) => {
          const widget = WIDGETS.find((w) => w.id === id);
          if (!widget) return null;
          return (
            <div key={id} className={`widget-cell span-${widget.span}${editing ? " editing" : ""}`}>
              {editing && (
                <div className="widget-controls">
                  <span className="widget-name">{widget.label}</span>
                  <button
                    className="widget-btn"
                    title="Monter"
                    disabled={i === 0}
                    onClick={() => moveWidget(i, -1)}
                  >
                    ↑
                  </button>
                  <button
                    className="widget-btn"
                    title="Descendre"
                    disabled={i === layout.length - 1}
                    onClick={() => moveWidget(i, 1)}
                  >
                    ↓
                  </button>
                  <button
                    className="widget-btn danger"
                    title="Retirer"
                    onClick={() => removeWidget(id)}
                  >
                    ×
                  </button>
                </div>
              )}
              {widget.render(ctx)}
            </div>
          );
        })}
      </div>
    </div>
  );
}

export default DashboardPage;
