const API_URL = "http://localhost:3000";

export async function fetchAlerts() {

  const token =
    localStorage.getItem("token");

  const response = await fetch(
    `${API_URL}/alerts`,
    {
      headers: {
        Authorization: `Bearer ${token}`,
      },
    }
  );

  return response.json();
}
export async function fetchLogs() {

  const token =
    localStorage.getItem("token");

  const response = await fetch(
    `${API_URL}/logs`,
    {
      headers: {
        Authorization: `Bearer ${token}`,
      },
    }
  );

  return response.json();
}