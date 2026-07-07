import { useEffect, useMemo, useState } from "react";
import { fetchAlerts, fetchLogs } from "../services/api";
import SeverityBadge, { normalizeSeverity } from "../components/SeverityBadge";

const SEVERITY_COLORS: Record<string, string> = {
  critical: "#ef4444",
  high: "#f97316",
  medium: "#eab308",
  low: "#3b82f6",
  info: "#64748b",
};

/* Extrait les techniques MITRE (T1110, T1021.001…) présentes dans le message. */
function extractMitre(message = ""): string[] {
  return message.match(/T\d{4}(?:\.\d{3})?/g) ?? [];
}

const GRANULARITIES = [
  { key: "hour", label: "Heure" },
  { key: "day", label: "Jour" },
  { key: "month", label: "Mois" },
  { key: "year", label: "Année" },
];

const SEV_ORDER = ["critical", "high", "medium", "low", "info"];

/* Ramène une date au début de son créneau (heure/jour/mois/année). */
function bucketStart(d: Date, g: string): Date {
  const x = new Date(d);
  x.setMilliseconds(0);
  x.setSeconds(0);
  x.setMinutes(0);
  if (g === "hour") return x;
  x.setHours(0);
  if (g === "day") return x;
  x.setDate(1);
  if (g === "month") return x;
  x.setMonth(0);
  return x;
}

/* Avance/recule d'un créneau. */
function stepBucket(d: Date, g: string, dir: number): Date {
  const x = new Date(d);
  if (g === "hour") x.setHours(x.getHours() + dir);
  else if (g === "day") x.setDate(x.getDate() + dir);
  else if (g === "month") x.setMonth(x.getMonth() + dir);
  else x.setFullYear(x.getFullYear() + dir);
  return x;
}

function captionFor(g: string): string {
  if (g === "hour") return "1 heure par colonne";
  if (g === "day") return "1 jour par colonne";
  if (g === "month") return "1 mois par colonne";
  return "1 an par colonne";
}

function shortLabel(d: Date, g: string): string {
  const p = (n: number) => String(n).padStart(2, "0");
  if (g === "hour") return `${p(d.getHours())}h`;
  if (g === "day") return `${p(d.getDate())}/${p(d.getMonth() + 1)}`;
  if (g === "month") return `${p(d.getMonth() + 1)}/${d.getFullYear()}`;
  return `${d.getFullYear()}`;
}

type Bucket = { start: Date; total: number; sev: Record<string, number>; sample: any };

/* Histogramme du volume d'alertes façon Splunk : axe temporel CONTIGU (créneaux
   vides inclus), barres empilées par sévérité, axe Y + grille, granularité. */
function Timeline({ alerts, onSelect }: { alerts: any[]; onSelect: (a: any) => void }) {
  const [granularity, setGranularity] = useState("hour");

  const buckets = useMemo<Bucket[]>(() => {
    if (alerts.length === 0) return [];

    // 1. Agrège les alertes par créneau.
    const counts = new Map<number, Bucket>();
    let minT = Infinity;
    let maxT = -Infinity;
    for (const a of alerts) {
      const bs = bucketStart(new Date(a.created_at), granularity).getTime();
      minT = Math.min(minT, bs);
      maxT = Math.max(maxT, bs);
      const e = counts.get(bs) ?? { start: new Date(bs), total: 0, sev: {}, sample: a };
      e.total += 1;
      const s = normalizeSeverity(a.severity);
      e.sev[s] = (e.sev[s] ?? 0) + 1;
      counts.set(bs, e);
    }

    // 2. Étend l'axe jusqu'au créneau courant (comme un vrai SIEM).
    maxT = Math.max(maxT, bucketStart(new Date(), granularity).getTime());

    // 3. Génère TOUS les créneaux contigus (vides compris).
    const list: Bucket[] = [];
    let cur = new Date(minT);
    while (cur.getTime() <= maxT && list.length < 400) {
      list.push(counts.get(cur.getTime()) ?? { start: new Date(cur), total: 0, sev: {}, sample: null });
      cur = stepBucket(cur, granularity, 1);
    }

    // 4. Garantit un minimum de colonnes (remplit le passé) et plafonne.
    while (list.length < 16) {
      list.unshift({ start: stepBucket(list[0].start, granularity, -1), total: 0, sev: {}, sample: null });
    }
    return list.length > 60 ? list.slice(list.length - 60) : list;
  }, [alerts, granularity]);

  const max = Math.max(...buckets.map((b) => b.total), 1);
  const step = Math.max(1, Math.ceil(buckets.length / 8));
  const gridVals = [max, Math.round(max / 2), 0];

  return (
    <div className="panel timeline-wrap">
      <div className="timeline-head">
        <h3 className="panel-title" style={{ margin: 0 }}>
          Volume d'alertes
        </h3>
        <div className="seg-toggle">
          {GRANULARITIES.map((g) => (
            <button
              key={g.key}
              className={`seg-opt${granularity === g.key ? " active" : ""}`}
              onClick={() => setGranularity(g.key)}
            >
              {g.label}
            </button>
          ))}
        </div>
      </div>

      {buckets.length === 0 ? (
        <div className="empty-state">Aucune alerte</div>
      ) : (
        <>
          <div className="hist-caption">{captionFor(granularity)}</div>
          <div className="hist-plot">
            <div className="hist-grid-lines">
              {gridVals.map((val, i) => (
                <div className="hist-grid-line" key={i} style={{ bottom: `${100 - i * 50}%` }}>
                  <span>{val}</span>
                </div>
              ))}
            </div>
            <div className="histogram">
              {buckets.map((b, i) => (
                <div
                  className="hist-col"
                  key={i}
                  onClick={() => b.sample && onSelect(b.sample)}
                  title={`${shortLabel(b.start, granularity)} — ${b.total} alerte${b.total > 1 ? "s" : ""}`}
                >
                  {b.total > 0 && (
                    <div className="hist-bar" style={{ height: `${(b.total / max) * 100}%` }}>
                      {SEV_ORDER.map((s) =>
                        b.sev[s] ? (
                          <div key={s} style={{ flexGrow: b.sev[s], background: SEVERITY_COLORS[s] }} />
                        ) : null
                      )}
                    </div>
                  )}
                </div>
              ))}
            </div>
          </div>
          <div className="hist-xaxis">
            {buckets.map((b, i) => (
              <span className="hist-xlabel" key={i}>
                {i % step === 0 ? shortLabel(b.start, granularity) : ""}
              </span>
            ))}
          </div>
        </>
      )}
    </div>
  );
}

/* Panneau de détail (drawer) façon Splunk. */
function AlertDrawer({ alert, onClose }: { alert: any; onClose: () => void }) {
  const [relatedLogs, setRelatedLogs] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    setLoading(true);
    fetchLogs(alert.source_name ?? "")
      .then((data) => setRelatedLogs(Array.isArray(data) ? data.slice(0, 10) : []))
      .catch(() => setRelatedLogs([]))
      .finally(() => setLoading(false));
  }, [alert]);

  const mitre = extractMitre(alert.message);

  return (
    <>
      <div className="drawer-overlay" onClick={onClose} />
      <aside className="drawer">
        <header className="drawer-header">
          <div>
            <div className="drawer-title">{alert.rule_name}</div>
            <SeverityBadge severity={alert.severity} />
          </div>
          <button className="drawer-close" onClick={onClose} title="Fermer">
            ✕
          </button>
        </header>

        <div className="drawer-body">
          <div className="detail-grid">
            <div className="detail-row">
              <span className="detail-label">Source</span>
              <span className="cell-mono">{alert.source_name ?? "—"}</span>
            </div>
            <div className="detail-row">
              <span className="detail-label">Horodatage</span>
              <span className="cell-mono">{new Date(alert.created_at).toLocaleString()}</span>
            </div>
            <div className="detail-row">
              <span className="detail-label">Description</span>
              <span>{alert.message}</span>
            </div>
          </div>

          {mitre.length > 0 && (
            <div className="drawer-section">
              <div className="drawer-section-title">MITRE ATT&amp;CK</div>
              <div className="mitre-chips">
                {mitre.map((t) => (
                  <a
                    key={t}
                    className="mitre-chip"
                    href={`https://attack.mitre.org/techniques/${t.replace(".", "/")}`}
                    target="_blank"
                    rel="noreferrer"
                  >
                    {t}
                  </a>
                ))}
              </div>
            </div>
          )}

          <div className="drawer-section">
            <div className="drawer-section-title">
              Événements récents de la source
              <span className="drawer-hint"> — contexte / preuve</span>
            </div>
            {loading && <div className="empty-state">Chargement…</div>}
            {!loading && relatedLogs.length === 0 && (
              <div className="empty-state">Aucun événement associé</div>
            )}
            {!loading && relatedLogs.length > 0 && (
              <div className="related-list">
                {relatedLogs.map((log) => (
                  <div className="related-log" key={log.id}>
                    <div className="related-log-head">
                      <SeverityBadge severity={log.severity} />
                      <span className="cell-mono related-log-type">{log.event_type}</span>
                      <span className="cell-mono related-log-time">
                        {new Date(log.created_at).toLocaleTimeString()}
                      </span>
                    </div>
                    <div className="related-log-msg">{log.message}</div>
                  </div>
                ))}
              </div>
            )}
          </div>
        </div>
      </aside>
    </>
  );
}

function AlertsPage() {
  const [alerts, setAlerts] = useState<any[]>([]);
  const [search, setSearch] = useState("");
  const [severityFilter, setSeverityFilter] = useState("all");
  const [selected, setSelected] = useState<any | null>(null);

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
      <Timeline alerts={filtered} onSelect={setSelected} />

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
              <th>Description</th>
              <th>Horodatage</th>
            </tr>
          </thead>
          <tbody>
            {filtered.map((alert) => (
              <tr
                key={alert.id}
                className={`expandable${selected?.id === alert.id ? " row-selected" : ""}`}
                onClick={() => setSelected(alert)}
              >
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

      {selected && <AlertDrawer alert={selected} onClose={() => setSelected(null)} />}
    </div>
  );
}

export default AlertsPage;
