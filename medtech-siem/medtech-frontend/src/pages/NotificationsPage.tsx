import { useEffect, useState } from "react";
import {
  fetchAlertConfigs,
  createAlertConfig,
  deleteAlertConfig,
} from "../services/api";

/* Libellés lisibles pour les noms de règles techniques du backend. */
const RULE_LABELS: Record<string, string> = {
  multiple_failed_logins: "Brute-force ciblé (échecs répétés)",
  password_spray: "Password spraying",
  bruteforce_success: "Brute-force réussi",
  rdp_bruteforce_external: "Brute-force RDP (Internet)",
  powershell_suspect: "PowerShell suspect",
  cmd_suspect: "cmd.exe suspect",
  network_scan: "Scan réseau",
  ransomware: "Ransomware",
  data_exfiltration: "Exfiltration de données",
  privilege_escalation: "Élévation de privilèges",
  rdp_foreign_country: "RDP pays inhabituel",
  impossible_travel: "Impossible travel",
  new_account_admin_group: "Compte créé + promu admin",
  audit_log_cleared: "Journal d'audit effacé",
  shadow_copy_deletion: "Suppression shadow copies",
  defense_disabled: "Défenses désactivées",
  office_spawns_shell: "Office engendre un shell",
};

const ruleLabel = (r: string) => RULE_LABELS[r] ?? r;

export default function NotificationsPage() {
  const [configs, setConfigs] = useState<any[]>([]);
  const [rules, setRules] = useState<string[]>([]);

  const [rule, setRule] = useState("");
  const [threshold, setThreshold] = useState(5);
  const [windowMin, setWindowMin] = useState(10);
  const [comment, setComment] = useState("");
  const [error, setError] = useState("");
  const [saving, setSaving] = useState(false);

  async function load() {
    try {
      const data = await fetchAlertConfigs();
      setConfigs(Array.isArray(data.configs) ? data.configs : []);
      setRules(Array.isArray(data.rules) ? data.rules : []);
      if (!rule && data.rules?.length) setRule(data.rules[0]);
    } catch (e) {
      console.error(e);
    }
  }

  useEffect(() => {
    load();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  async function handleCreate(e: React.FormEvent) {
    e.preventDefault();
    setError("");
    if (!comment.trim()) {
      setError("Le commentaire est obligatoire.");
      return;
    }
    setSaving(true);
    try {
      const res = await createAlertConfig(rule, threshold, windowMin, comment.trim());
      if (res?.error) {
        setError(res.error);
      } else {
        setComment("");
        await load();
      }
    } catch {
      setError("Création impossible (serveur injoignable ?).");
    } finally {
      setSaving(false);
    }
  }

  async function handleDelete(id: string) {
    await deleteAlertConfig(id);
    await load();
  }

  return (
    <div>
      <div className="banner">
        Creation des règles de notification : la regle de detection
        sera envoyé à <strong>medtechsiem@gmail.com</strong> en cas d'activation.
      </div>

      <div className="dashboard-grid">
        <div className="panel">
          <h3 className="panel-title">Nouvelle règle de notification</h3>
          <form onSubmit={handleCreate}>
            <div className="login-field">
              <label>Règle à surveiller</label>
              <select
                className="filter-select"
                style={{ width: "100%" }}
                value={rule}
                onChange={(e) => setRule(e.target.value)}
              >
                {rules.map((r) => (
                  <option key={r} value={r}>
                    {ruleLabel(r)}
                  </option>
                ))}
              </select>
            </div>

            <div style={{ display: "flex", gap: 12 }}>
              <div className="login-field" style={{ flex: 1 }}>
                <label>Nombre d'apparitions</label>
                <input
                  type="number"
                  min={1}
                  max={1000}
                  value={threshold}
                  onChange={(e) => setThreshold(Number(e.target.value))}
                />
              </div>
              <div className="login-field" style={{ flex: 1 }}>
                <label>Fenêtre (minutes)</label>
                <input
                  type="number"
                  min={1}
                  max={10080}
                  value={windowMin}
                  onChange={(e) => setWindowMin(Number(e.target.value))}
                />
              </div>
            </div>

            <div className="login-field">
              <label>Commentaire</label>
              <textarea
                className="contact-textarea"
                rows={4}
                placeholder="Ex. cmd.exe suspect répété = reconnaissance post-compromission, isoler le poste."
                value={comment}
                onChange={(e) => setComment(e.target.value)}
              />
            </div>

            {error && <p className="login-error">{error}</p>}

            <button
              type="submit"
              className="btn btn-primary"
              style={{ width: "100%", justifyContent: "center" }}
              disabled={saving}
            >
              {saving ? "Enregistrement…" : "Créer la règle de notification"}
            </button>
          </form>
        </div>

        <div className="panel">
          <h3 className="panel-title">Règles configurées ({configs.length})</h3>
          {configs.length === 0 ? (
            <div className="empty-state">Aucune règle de notification</div>
          ) : (
            <div className="related-list">
              {configs.map((c) => (
                <div className="related-log" key={c.id}>
                  <div className="related-log-head">
                    <span className="cell-strong">{ruleLabel(c.rule_name)}</span>
                    <span className="related-log-time">
                      ≥ {c.threshold} / {c.window_minutes} min
                    </span>
                    <button
                      className="widget-btn danger"
                      title="Supprimer"
                      style={{ marginLeft: 8 }}
                      onClick={() => handleDelete(c.id)}
                    >
                      ✕
                    </button>
                  </div>
                  <div className="related-log-msg">{c.comment}</div>
                </div>
              ))}
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
