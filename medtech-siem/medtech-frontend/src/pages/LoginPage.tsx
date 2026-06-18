import { useState } from "react";

export default function LoginPage() {

  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");

  async function handleLogin() {

    const response = await fetch(
      "http://localhost:3000/auth/login",
      {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          username,
          password,
        }),
      }
    );

    const data = await response.json();

    console.log(data);

    localStorage.setItem(
      "token",
      data.token
    );

    localStorage.setItem(
      "role",
      data.role
    );

    window.location.href = "/dashboard";
  }

  return (
    <div style={{ padding: 40 }}>

      <h1>MedTech SIEM Login</h1>

      <input
        placeholder="username"
        value={username}
        onChange={(e) =>
          setUsername(e.target.value)
        }
      />

      <br />
      <br />

      <input
        type="password"
        placeholder="password"
        value={password}
        onChange={(e) =>
          setPassword(e.target.value)
        }
      />

      <br />
      <br />

      <button onClick={handleLogin}>
        Login
      </button>

    </div>
  );
}