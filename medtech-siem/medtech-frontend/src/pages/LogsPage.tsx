import { useEffect, useState } from "react";
import { fetchLogs } from "../services/api";

function LogsPage() {
  const [logs, setLogs] = useState<any[]>([]);

  useEffect(() => {
    fetchLogs().then(setLogs);
  }, []);

  return (
    <div>
      <h2>Logs</h2>

      {logs.map((log) => (
        <div
          key={log.id}
          style={{
            border: "1px solid gray",
            padding: "10px",
            marginBottom: "10px",
          }}
        >
          <p><strong>Source:</strong> {log.source_name}</p>
          <p><strong>Type:</strong> {log.event_type}</p>
          <p><strong>Severity:</strong> {log.severity}</p>
          <p><strong>Message:</strong> {log.message}</p>
        </div>
      ))}
    </div>
  );
}

export default LogsPage;