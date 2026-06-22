import { useEffect, useMemo, useState } from "react";
import { fetchAlerts } from "../services/api";
import SeverityBadge, { normalizeSeverity } from "../components/SeverityBadge";

function AlertsPage() {
  const [alerts, setAlerts] = useState<any[]>([]);
  const [search, setSearch] = useState("");
  const [severityFilter, setSeverityFilter] = useState("all");

  const load = () => {
    fetchAlerts()
      .then((data) => setAlerts(Array.isArray(data) ? data : []))
      .catch(() => {});
  };

  useEffect(() => {
    load();
    const interval = setInterval(load, 10_000);
    return () => clearInterval(interval);
  }, []);

  const filtered = useMemo(() => {
    const q = search.toLowerCase();
    return alerts.filter((alert) => {
      if (severityFilter !== "all" && normalizeSeverity(alert.severity) !== severityFilter) {
        return false;
      }
      if (!q) return true;
      return [alert.rule_name, alert.message, alert.source_name]
        .filter(Boolean)
        .some((field: string) => field.toLowerCase().includes(q));
    });
  }, [alerts, search, severityFilter]);

  return (
    <div>
      <div className="toolbar">
        <input
          className="search-input"
          placeholder="Rechercher une règle, source, message…"
          value={search}
          onChange={(e) => setSearch(e.target.value)}
        />
        <select
          className="filter-select"
          value={severityFilter}
          onChange={(e) => setSeverityFilter(e.target.value)}
        >
          <option value="all">Toutes sévérités</option>
          <option value="critical">Critical</option>
          <option value="high">High</option>
          <option value="medium">Medium</option>
          <option value="low">Low</option>
          <option value="info">Info</option>
        </select>
        <button className="btn" onClick={load}>
          Actualiser
        </button>
        <span style={{ color: "var(--text-faint)", fontSize: 12 }}>
          {filtered.length} alerte{filtered.length > 1 ? "s" : ""}
        </span>
      </div>

      <div className="table-wrap">
        <table className="data-table">
          <thead>
            <tr>
              <th>Règle</th>
              <th>Sévérité</th>
              <th>Source</th>
              <th>Message</th>
              <th>Horodatage</th>
            </tr>
          </thead>
          <tbody>
            {filtered.map((alert) => (
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
            {filtered.length === 0 && (
              <tr>
                <td colSpan={5}>
                  <div className="empty-state">Aucune alerte ne correspond aux critères</div>
                </td>
              </tr>
            )}
          </tbody>
        </table>
      </div>
    </div>
  );
}

export default AlertsPage;
