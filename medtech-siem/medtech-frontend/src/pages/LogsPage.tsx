import { Fragment, useEffect, useState } from "react";
import { fetchLogs } from "../services/api";
import SeverityBadge from "../components/SeverityBadge";

const REFRESH_OPTIONS = [
  { value: "0", label: "Auto-refresh OFF" },
  { value: "10000", label: "10 secondes" },
  { value: "60000", label: "1 minute" },
  { value: "300000", label: "5 minutes" },
  { value: "600000", label: "10 minutes" },
];

export default function LogsPage() {
  const [logs, setLogs] = useState<any[]>([]);
  const [search, setSearch] = useState("");
  const [severity, setSeverity] = useState("");
  const [refreshInterval, setRefreshInterval] = useState("0");
  const [lastRefresh, setLastRefresh] = useState<Date | null>(null);
  const [expandedId, setExpandedId] = useState<string | null>(null);

  async function loadLogs() {
    try {
      const data = await fetchLogs(search, severity);
      setLogs(Array.isArray(data) ? data : []);
      setLastRefresh(new Date());
    } catch (err) {
      console.error(err);
    }
  }

  useEffect(() => {
    loadLogs();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  useEffect(() => {
    if (refreshInterval === "0") return;
    const interval = setInterval(loadLogs, Number(refreshInterval));
    return () => clearInterval(interval);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [refreshInterval, search, severity]);

  const autoOn = refreshInterval !== "0";

  return (
    <div>
      <div className="toolbar">
        <input
          className="search-input"
          placeholder="Rechercher (message, utilisateur, IP, vendor, source…)"
          value={search}
          onChange={(e) => setSearch(e.target.value)}
          onKeyDown={(e) => e.key === "Enter" && loadLogs()}
        />
        <select
          className="filter-select"
          value={severity}
          onChange={(e) => setSeverity(e.target.value)}
        >
          <option value="">Toutes sévérités</option>
          <option value="critical">Critical</option>
          <option value="high">High</option>
          <option value="medium">Medium</option>
          <option value="low">Low</option>
        </select>
        <button className="btn btn-primary" onClick={loadLogs}>
          Rechercher
        </button>
        <select
          className="filter-select"
          value={refreshInterval}
          onChange={(e) => setRefreshInterval(e.target.value)}
        >
          {REFRESH_OPTIONS.map((o) => (
            <option key={o.value} value={o.value}>
              {o.label}
            </option>
          ))}
        </select>

        <div className="refresh-status">
          <span className={`status-dot${autoOn ? " on" : ""}`} />
          {autoOn ? `Auto ${Number(refreshInterval) / 1000}s` : "Manuel"}
          {lastRefresh && (
            <span className="refresh-time">· {lastRefresh.toLocaleTimeString()}</span>
          )}
        </div>
      </div>

      <div className="table-wrap">
        <table className="data-table">
          <thead>
            <tr>
              <th style={{ width: 160 }}>Horodatage</th>
              <th>Vendor</th>
              <th>Host</th>
              <th>Utilisateur</th>
              <th>IP</th>
              <th>Source</th>
              <th>Événement</th>
              <th>Sévérité</th>
              <th>Message</th>
            </tr>
          </thead>
          <tbody>
            {logs.map((log) => (
              <Fragment key={log.id}>
                <tr
                  className="expandable"
                  onClick={() => setExpandedId(expandedId === log.id ? null : log.id)}
                >
                  <td className="cell-mono">{new Date(log.created_at).toLocaleString()}</td>
                  <td>{log.vendor ? <span className="tag">{log.vendor}</span> : "—"}</td>
                  <td className="cell-mono">{log.hostname ?? "—"}</td>
                  <td className="cell-mono">{log.username ?? "—"}</td>
                  <td className="cell-mono">{log.ip_address ?? "—"}</td>
                  <td className="cell-strong cell-mono">{log.source_name}</td>
                  <td className="cell-mono">{log.event_type}</td>
                  <td>
                    <SeverityBadge severity={log.severity} />
                  </td>
                  <td>{log.message}</td>
                </tr>
                {expandedId === log.id && (
                  <tr>
                    <td colSpan={9}>
                      <div className="raw-log">{JSON.stringify(log.raw_log ?? log, null, 2)}</div>
                    </td>
                  </tr>
                )}
              </Fragment>
            ))}
            {logs.length === 0 && (
              <tr>
                <td colSpan={9}>
                  <div className="empty-state">Aucun log ne correspond aux critères</div>
                </td>
              </tr>
            )}
          </tbody>
        </table>
      </div>
    </div>
  );
}
