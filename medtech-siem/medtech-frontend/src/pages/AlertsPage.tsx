import { useEffect, useState } from "react";
import { fetchAlerts } from "../services/api";

function AlertsPage() {
  const [alerts, setAlerts] = useState<any[]>([]);

  useEffect(() => {
    fetchAlerts().then(setAlerts);
  }, []);

  return (
    <div>
      <h2>Alertes</h2>

      {alerts.map((alert) => (
        <div
          key={alert.id}
          style={{
            border: "1px solid gray",
            padding: "10px",
            marginBottom: "10px",
          }}
        >
          <p><strong>Rule:</strong> {alert.rule_name}</p>
          <p><strong>Severity:</strong> {alert.severity}</p>
          <p><strong>Source:</strong> {alert.source_name}</p>
          <p><strong>Message:</strong> {alert.message}</p>
        </div>
      ))}
    </div>
  );
}

export default AlertsPage;