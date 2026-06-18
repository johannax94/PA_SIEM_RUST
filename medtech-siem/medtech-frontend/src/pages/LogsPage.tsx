import { useEffect, useState } from "react";
import { fetchLogs } from "../services/api";

export default function LogsPage() {

  const [logs, setLogs] =
    useState<any[]>([]);

  const [selectedLog, setSelectedLog] =
    useState<any | null>(null);

  const [search, setSearch] =
    useState("");

  const [severity, setSeverity] =
    useState("");

  const [refreshInterval, setRefreshInterval] =
    useState("0");

  const [lastRefresh, setLastRefresh] =
    useState(new Date());

  async function loadLogs() {

    try {

      const data = await fetchLogs(
        search,
        severity
      );

      setLogs(data);

      // Met à jour l'heure du dernier rafraîchissement
      setLastRefresh(new Date());

    } catch (err) {

      console.error(err);

    }
  }

  useEffect(() => {

    loadLogs();

  }, []);

  useEffect(() => {

    if (refreshInterval === "0") {
      return;
    }

    const interval = setInterval(() => {

      loadLogs();

    }, Number(refreshInterval));

    return () => clearInterval(interval);

  }, [
    refreshInterval,
    search,
    severity,
  ]);


  function severityColor(severity: string) {
    switch (severity?.toLowerCase()) {
      case "critical":
        return "#ff1744";

      case "high":
        return "#ff9800";

      case "medium":
        return "#ffd54f";

      case "low":
        return "#4caf50";

      default:
        return "#90a4ae";
    }
  }

  function vendorColor(vendor: string) {
    switch (vendor?.toLowerCase()) {
      case "windows":
        return "#1976d2";

      case "fortinet":
        return "#d32f2f";

      case "cisco":
        return "#00acc1";

      case "linux":
        return "#fbc02d";

      case "vmware":
        return "#8e24aa";

      default:
        return "#546e7a";
    }
  }

  return (
    <div
      style={{
        background: "#0f172a",
        minHeight: "100vh",
        color: "white",
        padding: 30,
      }}
    >
      <h1 style={{ marginBottom: 25 }}>
        📄 Logs
      </h1>
              <p
          style={{
            color: "#94a3b8",
            marginTop: -10,
            marginBottom: 20,
            fontSize: 14,
          }}
        >
          Dernière mise à jour :
          {" "}
          {lastRefresh.toLocaleTimeString()}
        </p>

      <div
        style={{
          display: "flex",
          gap: 15,
          marginBottom: 25,
        }}
      >
        <input
          placeholder="Recherche (message, utilisateur, IP, vendor...)"
          value={search}
          onChange={(e) =>
            setSearch(e.target.value)
          }
          style={{
            flex: 1,
            padding: 12,
            borderRadius: 8,
            border: "1px solid #334155",
            background: "#1e293b",
            color: "white",
          }}
        />

        <select
          value={severity}
          onChange={(e) =>
            setSeverity(e.target.value)
          }
          style={{
            padding: 12,
            borderRadius: 8,
            background: "#1e293b",
            color: "white",
            border: "1px solid #334155",
          }}
        >
          <option value="">
            Toutes
          </option>

          <option value="low">
            Low
          </option>

          <option value="medium">
            Medium
          </option>

          <option value="high">
            High
          </option>

          <option value="critical">
            Critical
          </option>
        </select>
            
      <button
        onClick={loadLogs}
        style={{
          padding: "12px 22px",
          border: "none",
          borderRadius: 8,
          background: "#2563eb",
          color: "white",
          cursor: "pointer",
          fontWeight: "bold",
        }}
      >
        🔍 Rechercher
      </button>
            <select
        value={refreshInterval}
        onChange={(e) =>
          setRefreshInterval(e.target.value)
        }
        style={{
          padding: 12,
          borderRadius: 8,
          background: "#1e293b",
          color: "white",
          border: "1px solid #334155",
          minWidth: 180,
        }}
      >
        <option value="0">
          Auto-refresh OFF
        </option>

        <option value="60000">
          1 minute
        </option>

        <option value="300000">
          5 minutes
        </option>

        <option value="600000">
          10 minutes
        </option>

        <option value="3600000">
          60 minutes
        </option>
      </select>


      <div
        style={{
          marginLeft: "auto",
          display: "flex",
          alignItems: "center",
          gap: 10,
        }}
      >
        <div
          style={{
            width: 12,
            height: 12,
            borderRadius: "50%",
            background:
              refreshInterval === "0"
                ? "#ef4444"
                : "#22c55e",
          }}
        />

        <span
          style={{
            color: "#cbd5e1",
            fontWeight: 600,
            fontSize: 14,
          }}
        >
          {refreshInterval === "0"
            ? "Auto-refresh OFF"
            : `Auto-refresh ${Number(refreshInterval) / 1000}s`}
        </span>
      </div>

      </div>

      <div
        style={{
          background: "#111827",
          borderRadius: 12,
          overflow: "hidden",
          border: "1px solid #334155",
        }}
      >
        <table
          style={{
            width: "100%",
            borderCollapse: "collapse",
          }}
        >
          <thead
            style={{
              background: "#1e293b",
            }}
          >
            <tr>
              <th style={th}>Date</th>
              <th style={th}>Vendor</th>
              <th style={th}>Host</th>
              <th style={th}>Utilisateur</th>
              <th style={th}>IP</th>
              <th style={th}>Source</th>
              <th style={th}>Evènement</th>
              <th style={th}>Severity</th>
              <th style={th}>Message</th>
            </tr>
          </thead>

          <tbody>
            {logs.map((log) => (
              <tr
                key={log.id}
                onClick={() =>
                  setSelectedLog(log)
                }
                style={{
                  cursor: "pointer",
                  borderBottom:
                    "1px solid #1e293b",
                }}
              >
                <td style={td}>
                  {new Date(
                    log.created_at
                  ).toLocaleString()}
                </td>

                <td style={td}>
                  <span
                    style={{
                      background:
                        vendorColor(
                          log.vendor
                        ),
                      padding:
                        "4px 10px",
                      borderRadius: 20,
                      fontSize: 12,
                    }}
                  >
                    {log.vendor ?? "-"}
                  </span>
                </td>

                <td style={td}>
                  {log.hostname ?? "-"}
                </td>

                <td style={td}>
                  {log.username ?? "-"}
                </td>

                <td style={td}>
                  {log.ip_address ?? "-"}
                </td>

                <td style={td}>
                  {log.source_name}
                </td>

                <td style={td}>
                  {log.event_type}
                </td>

                <td style={td}>
                  <span
                    style={{
                      background:
                        severityColor(
                          log.severity
                        ),
                      color: "black",
                      padding:
                        "4px 10px",
                      borderRadius: 20,
                      fontWeight: "bold",
                    }}
                  >
                    {log.severity}
                  </span>
                </td>

                <td style={td}>
                  {log.message}
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>

      {selectedLog && (
        <div
          style={{
            marginTop: 30,
            background: "#111827",
            borderRadius: 10,
            padding: 20,
            border: "1px solid #334155",
          }}
        >
          <h2>
            🔍 Détails du log
          </h2>

          <pre
            style={{
              overflowX: "auto",
              color: "#4ade80",
            }}
          >
            {JSON.stringify(
              selectedLog,
              null,
              2
            )}
          </pre>
        </div>
      )}
    </div>
  );
}

const th = {
  padding: "15px",
  textAlign: "left" as const,
  fontWeight: "bold",
};

const td = {
  padding: "14px",
};