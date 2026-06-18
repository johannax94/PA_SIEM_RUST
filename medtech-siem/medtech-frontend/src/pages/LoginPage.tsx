import { useState } from "react";
import Logo from "../components/Logo";

export default function LoginPage() {
  const [uid, setUid] = useState("");
  const [password, setPassword] = useState("");
  const [error, setError] = useState("");
  const [loading, setLoading] = useState(false);

  async function handleLogin(e: React.FormEvent) {
    e.preventDefault();
    setError("");
    setLoading(true);

    try {
      const response = await fetch("http://localhost:3000/auth/login", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ username: uid, password }),
      });

      if (!response.ok) {
        throw new Error("bad credentials");
      }

      const data = await response.json();
      localStorage.setItem("token", data.token);
      window.location.href = "/";
    } catch {
      setError("Identifiants invalides ou serveur injoignable.");
      setLoading(false);
    }
  }

  return (
    <div className="login-bg">
      <form className="login-card" onSubmit={handleLogin}>
        <div className="login-logo">
          <Logo size={48} />
          <h1>MEDTECH</h1>
        </div>

        <div className="login-field">
          <label>UID</label>
          <input
            placeholder="uid"
            value={uid}
            autoFocus
            onChange={(e) => setUid(e.target.value)}
          />
        </div>

        <div className="login-field">
          <label>Mot de passe</label>
          <input
            type="password"
            placeholder="••••••••"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
          />
        </div>

        {error && <p className="login-error">{error}</p>}

        <button
          type="submit"
          className="btn btn-primary"
          style={{ width: "100%", justifyContent: "center" }}
          disabled={loading}
        >
          {loading ? "Connexion…" : "Se connecter"}
        </button>
      </form>
    </div>
  );
}
