import { useEffect, useState } from "react";
import { fetchAlerts } from "../services/api";

function AlertsPage() {

  const [alerts, setAlerts] =
    useState<any[]>([]);

  useEffect(() => {
    fetchAlerts().then(setAlerts);
  }, []);

  return (
    <div
      style={{
        padding: "30px",
      }}
    >
      <h1>Alertes</h1>

      <table
        style={{
          width: "100%",
          borderCollapse: "collapse",
        }}
      >
        <thead>
          <tr>
            <th style={thStyle}>Date</th>
            <th style={thStyle}>Règle</th>
            <th style={thStyle}>Sévérité</th>
            <th style={thStyle}>Source</th>
            <th style={thStyle}>Message</th>
          </tr>
        </thead>

        <tbody>
          {alerts.map((alert) => (
            <tr key={alert.id}>
              <td style={tdStyle}>
                {new Date(
                  alert.created_at
                ).toLocaleString()}
              </td>

              <td style={tdStyle}>
                {alert.rule_name}
              </td>

              <td style={tdStyle}>
                <span
                  style={{
                    padding:
                      "4px 10px",
                    borderRadius:
                      "999px",
                    backgroundColor:
                      alert.severity === "high"
                        ? "#e60b0b"
                        : alert.severity === "medium"
                        ? "#fff0cc"
                        : "#ddffdd",
                  }}
                >
                  {alert.severity}
                </span>
              </td>

              <td style={tdStyle}>
                {alert.source_name}
              </td>

              <td style={tdStyle}>
                {alert.message}
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

const thStyle = {
  borderBottom: "2px solid #ddd",
  padding: "12px",
  textAlign: "left" as const,
};

const tdStyle = {
  borderBottom: "1px solid #eee",
  padding: "12px",
};

export default AlertsPage;