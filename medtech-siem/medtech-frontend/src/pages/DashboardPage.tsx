import { useEffect, useState } from "react";
import { fetchDashboard } from "../services/api";

export default function DashboardPage() {

  const [stats, setStats] =
    useState<any>(null);

  useEffect(() => {
    fetchDashboard()
      .then(setStats);
  }, []);

  if (!stats) {
    return <p>Chargement...</p>;
  }

  return (
    <div style={{ padding: 30 }}>

      <h1>Dashboard</h1>

      <div
        style={{
          display: "flex",
          gap: "20px",
        }}
      >

        <div style={card}>
          <h3>Logs</h3>
          <h2>{stats.total_logs}</h2>
        </div>

        <div style={card}>
          <h3>Alertes</h3>
          <h2>{stats.total_alerts}</h2>
        </div>

      </div>

    </div>
  );
}

const card = {
  border: "1px solid #ddd",
  padding: "20px",
  borderRadius: "8px",
  width: "200px",
};