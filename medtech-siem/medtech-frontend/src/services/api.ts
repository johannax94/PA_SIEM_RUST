const API_URL = "http://localhost:3000";

export async function fetchAlerts() {
  const response = await fetch(`${API_URL}/alerts`);
  return response.json();
}

export async function fetchLogs() {
  const response = await fetch(`${API_URL}/logs`);
  return response.json();
}