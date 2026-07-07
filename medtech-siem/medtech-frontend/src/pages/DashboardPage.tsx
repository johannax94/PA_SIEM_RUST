import { useEffect, useMemo, useState, type CSSProperties } from "react";
import { fetchDashboard, fetchAlerts } from "../services/api";
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

/* Donut: répartition des alertes par sévérité */
function SeverityDonut({ alerts }: { alerts: any[] }) {
  const counts = useMemo(() => {
    const acc: Record<string, number> = {};
    for (const a of alerts) {
      const level = normalizeSeverity(a.severity);
      acc[level] = (acc[level] ?? 0) + 1;
    }
    return acc;
  }, [alerts]);

  const total = alerts.length;
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
          alertes
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
        {total === 0 && <div className="legend-row">Aucune alerte</div>}
      </div>
    </div>
  );
}

export default function DashboardPage() {
  const [stats, setStats] = useState<any>(null);
  const [alerts, setAlerts] = useState<any[]>([]);

  useEffect(() => {
    const load = () => {
      fetchDashboard()
        .then(setStats)
        .catch(() => {});
      fetchAlerts()
        .then((d) => setAlerts(Array.isArray(d) ? d : []))
        .catch(() => {});
    };
    load();
    const id = setInterval(load, 10_000);
    return () => clearInterval(id);
  }, []);

  const highSeverity = alerts.filter((a) =>
    ["critical", "high"].includes(normalizeSeverity(a.severity))
  ).length;

  const recentAlerts = alerts.slice(0, 8);

  return (
    <div>
      <div className="stat-grid">
        <StatCard label="Événements collectés" value={stats?.total_logs ?? 0} color="#4d7ea8" />
        <StatCard label="Alertes actives" value={stats?.total_alerts ?? 0} color="#eab308" />
        <StatCard label="Alertes critiques / high" value={highSeverity} color="#ef4444" />
      </div>

      <div className="dashboard-grid">
        <div className="table-wrap">
          <table className="data-table">
            <thead>
              <tr>
                <th>Dernières alertes</th>
                <th>Sévérité</th>
                <th>Source</th>
                <th>Message</th>
                <th>Horodatage</th>
              </tr>
            </thead>
            <tbody>
              {recentAlerts.map((alert) => (
                <tr key={alert.id}>
                  <td className="cell-strong">{alert.rule_name}</td>
                  <td>
                    <SeverityBadge severity={alert.severity} />
                  </td>
                  <td className="cell-mono">{alert.source_name ?? "—"}</td>
                  <td>{alert.message ?? alert.description}</td>
                  <td className="cell-mono">{new Date(alert.created_at).toLocaleString()}</td>
                </tr>
              ))}
              {recentAlerts.length === 0 && (
                <tr>
                  <td colSpan={5}>
                    <div className="empty-state">Aucune alerte — système sain</div>
                  </td>
                </tr>
              )}
            </tbody>
          </table>
        </div>

        <div className="panel">
          <h3 className="panel-title">Répartition par sévérité</h3>
          <SeverityDonut alerts={alerts} />
        </div>
      </div>
    </div>
  );
}
