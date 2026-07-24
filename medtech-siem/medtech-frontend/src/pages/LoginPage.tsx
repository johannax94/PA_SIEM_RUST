import { useState } from "react";
import Logo from "../components/Logo";

/* Le backend renvoie le rôle dans le JWT (claims). On le décode pour
   piloter l'affichage (ex. menu Utilisateurs réservé aux admins). */
function decodeRole(token: string): string {
  try {
    const payload = JSON.parse(atob(token.split(".")[1]));
    return payload.role ?? "";
  } catch {
    return "";
  }
}

export default function LoginPage() {
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [error, setError] = useState("");
  const [loading, setLoading] = useState(false);

  async function handleLogin(e: React.FormEvent) {
    e.preventDefault();
    setError("");
    setLoading(true);

    try {
      // ANCIEN: fetch("http://localhost:3000/auth/login", ...) — URL en dur qui
      // cassait la prod ET forçait un appel cross-origin (CORS) en local.
      const response = await fetch(`${import.meta.env.VITE_API_URL ?? ""}/auth/login`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ username, password }),
      });

      if (!response.ok) {
        throw new Error("bad credentials");
      }

      const data = await response.json();
      localStorage.setItem("token", data.token);
      localStorage.setItem("role", data.role ?? decodeRole(data.token));
      window.location.href = "/dashboard";
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
          <span className="login-sub">SIEM pour PME</span>
        </div>

        <div className="login-field">
          <label>Identifiant</label>
          <input
            placeholder="username"
            value={username}
            autoFocus
            onChange={(e) => setUsername(e.target.value)}
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
