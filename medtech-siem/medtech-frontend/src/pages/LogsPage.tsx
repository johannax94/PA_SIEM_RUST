import { useEffect, useMemo, useState } from "react";
import { fetchLogs } from "../services/api";
import SeverityBadge, { normalizeSeverity } from "../components/SeverityBadge";

function LogsPage() {
  const [logs, setLogs] = useState<any[]>([]);
  const [search, setSearch] = useState("");
  const [severityFilter, setSeverityFilter] = useState("all");
  const [expandedId, setExpandedId] = useState<string | null>(null);

  const load = () => {
    fetchLogs().then((data) => setLogs(Array.isArray(data) ? data : []));
  };

  useEffect(() => {
    load();
    const interval = setInterval(load, 10_000);
    return () => clearInterval(interval);
  }, []);

  const filtered = useMemo(() => {
    const q = search.toLowerCase();
    return logs.filter((log) => {
      if (severityFilter !== "all" && normalizeSeverity(log.severity) !== severityFilter) {
        return false;
      }
      if (!q) return true;
      return [log.source_name, log.event_type, log.message]
        .filter(Boolean)
        .some((field: string) => field.toLowerCase().includes(q));
    });
  }, [logs, search, severityFilter]);

  return (
    <div>
      <div className="toolbar">
        <input
          className="search-input"
          placeholder="Rechercher par source, type d'événement, message…"
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
          {filtered.length} événement{filtered.length > 1 ? "s" : ""}
        </span>
      </div>

      <div className="table-wrap">
        <table className="data-table">
          <thead>
            <tr>
              <th style={{ width: 170 }}>Horodatage</th>
              <th>Source</th>
              <th>Type d'événement</th>
              <th>Sévérité</th>
              <th>Message</th>
            </tr>
          </thead>
          <tbody>
            {filtered.map((log) => (
              <>
                <tr
                  key={log.id}
                  className="expandable"
                  onClick={() => setExpandedId(expandedId === log.id ? null : log.id)}
                >
                  <td className="cell-mono">{new Date(log.created_at).toLocaleString()}</td>
                  <td className="cell-strong cell-mono">{log.source_name}</td>
                  <td className="cell-mono">{log.event_type}</td>
                  <td>
                    <SeverityBadge severity={log.severity} />
                  </td>
                  <td>{log.message}</td>
                </tr>
                {expandedId === log.id && (
                  <tr key={`${log.id}-raw`}>
                    <td colSpan={5}>
                      <div className="raw-log">{JSON.stringify(log.raw_log, null, 2)}</div>
                    </td>
                  </tr>
                )}
              </>
            ))}
            {filtered.length === 0 && (
              <tr>
                <td colSpan={5}>
                  <div className="empty-state">
                    Aucun log ne correspond aux critères
                  </div>
                </td>
              </tr>
            )}
          </tbody>
        </table>
      </div>
    </div>
  );
}

export default LogsPage;
