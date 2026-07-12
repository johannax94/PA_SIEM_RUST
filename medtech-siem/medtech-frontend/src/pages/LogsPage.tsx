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

/* Techniques MITRE (T1110, T1021.001…) présentes dans le message. */
function extractMitre(message = ""): string[] {
  return [...new Set(message.match(/T\d{4}(?:\.\d{3})?/g) ?? [])];
}

/* "il y a 3 min", "il y a 2 h"… à partir d'une date. */
function timeAgo(date: Date): string {
  const s = Math.floor((Date.now() - date.getTime()) / 1000);
  if (s < 60) return "à l'instant";
  if (s < 3600) return `il y a ${Math.floor(s / 60)} min`;
  if (s < 86400) return `il y a ${Math.floor(s / 3600)} h`;
  return `il y a ${Math.floor(s / 86400)} j`;
}

/* Une paire label / valeur de la grille d'identité. */
function Field({ label, value, mono = true }: { label: string; value?: string | null; mono?: boolean }) {
  return (
    <div className="detail-row">
      <span className="detail-label">{label}</span>
      <span className={mono ? "cell-mono" : undefined}>{value || "—"}</span>
    </div>
  );
}

/* Panneau détaillé affiché sous une ligne dépliée. */
function LogDetail({ log }: { log: any }) {
  const created = new Date(log.created_at);
  const mitre = extractMitre(log.message);
  const raw = log.raw_log && typeof log.raw_log === "object" ? log.raw_log : {};
  const rawEntries = Object.entries(raw);

  return (
    <div className="log-detail">
      <div className="log-detail-head">
        <span className="cell-mono cell-strong">{log.event_type}</span>
        <SeverityBadge severity={log.severity} />
        <span className="related-log-time">
          {created.toLocaleString()} · {timeAgo(created)}
        </span>
      </div>

      <div className="drawer-section-title">Identité</div>
      <div className="log-detail-grid">
        <Field label="Source" value={log.source_name} />
        <Field label="Vendor" value={log.vendor} mono={false} />
        <Field label="Host" value={log.hostname} />
        <Field label="Utilisateur" value={log.username} />
        <Field label="Adresse IP" value={log.ip_address} />
        <Field label="Sévérité" value={log.severity} mono={false} />
      </div>

      <div className="drawer-section-title">Message</div>
      <div className="log-detail-message">{log.message}</div>

      {mitre.length > 0 && (
        <>
          <div className="drawer-section-title">MITRE ATT&CK</div>
          <div className="mitre-chips">
            {mitre.map((t) => (
              <a
                key={t}
                className="mitre-chip"
                href={`https://attack.mitre.org/techniques/${t.replace(".", "/")}/`}
                target="_blank"
                rel="noreferrer"
                onClick={(e) => e.stopPropagation()}
              >
                {t}
              </a>
            ))}
          </div>
        </>
      )}

      <div className="drawer-section-title">
        Données brutes (raw_log)
        <button
          className="btn"
          style={{ marginLeft: 12, padding: "4px 10px", fontSize: 12 }}
          onClick={(e) => {
            e.stopPropagation();
            navigator.clipboard?.writeText(JSON.stringify(log.raw_log, null, 2));
          }}
        >
          Copier JSON
        </button>
      </div>
      {rawEntries.length > 0 ? (
        <div className="log-detail-grid">
          {rawEntries.map(([k, v]) => (
            <Field key={k} label={k} value={typeof v === "object" ? JSON.stringify(v) : String(v)} />
          ))}
        </div>
      ) : (
        <div className="empty-state" style={{ padding: 16 }}>
          Aucune donnée brute supplémentaire
        </div>
      )}
    </div>
  );
}

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

  // Recherche en direct : relance débouncée à chaque frappe / changement de
  // sévérité (et charge la liste au montage).
  useEffect(() => {
    const timer = setTimeout(loadLogs, 400);
    return () => clearTimeout(timer);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [search, severity]);

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
          placeholder="Recherche partielle : power, 192.168, admin… (plusieurs termes = ET)"
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
                    <td colSpan={9} style={{ padding: 0 }}>
                      <LogDetail log={log} />
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
