const API_URL =
    import.meta.env.VITE_API_URL;

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


export async function fetchLogs(
  search = "",
  severity = ""
){
  const token = localStorage.getItem("token");

  const params = new URLSearchParams();

  if (search) {
    params.append("q", search);
  }

  if (severity) {
    params.append("severity", severity);
  }

  const response = await fetch(
    `${API_URL}/logs?${params.toString()}`,
    {
      headers: {
        Authorization: `Bearer ${token}`,
      },
    }
  );

  return response.json();
}
export async function deleteUser(
  username: string
) {

  const token =
    localStorage.getItem("token");

  const response = await fetch(
    `${API_URL}/users/${username}`,
    {
      method: "DELETE",
      headers: {
        Authorization:
          `Bearer ${token}`,
      },
    }
  );

  return response.json();
}
export async function updateUserRole(
  username: string,
  role: string
) {

  const token =
    localStorage.getItem("token");

  const response = await fetch(
    `${API_URL}/users/${username}/role`,
    {
      method: "PUT",
      headers: {
        "Content-Type":
          "application/json",
        Authorization:
          `Bearer ${token}`,
      },
      body: JSON.stringify({
        role,
      }),
    }
  );

  return response.json();
}
export async function fetchDashboard() {

  const token =
    localStorage.getItem("token");

  const response = await fetch(
    `${API_URL}/dashboard`,
    {
      headers: {
        Authorization: `Bearer ${token}`,
      },
    }
  );

  return response.json();
}

export async function fetchUsers() {

  const token =
    localStorage.getItem("token");

  const response = await fetch(
    `${API_URL}/users`,
    {
      headers: {
        Authorization: `Bearer ${token}`,
      },
    }
  );

  return response.json();
}
export async function createUser(
  username: string,
  password: string,
  role: string
) {

  const token =
    localStorage.getItem("token");

  const response = await fetch(
    `${API_URL}/users`,
    {
      method: "POST",
      headers: {
        "Content-Type":
          "application/json",
        Authorization:
          `Bearer ${token}`,
      },
      body: JSON.stringify({
        username,
        password,
        role,
      }),
    }
  );

  return response.json();
}