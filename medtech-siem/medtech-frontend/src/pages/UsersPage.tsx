import { useEffect, useState } from "react";
import { fetchUsers, createUser, deleteUser, updateUserRole } from "../services/api";

const ROLES = ["admin", "analyst", "viewer"];

export default function UsersPage() {
  const [users, setUsers] = useState<any[]>([]);
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [newUserRole, setNewUserRole] = useState("analyst");
  const [message, setMessage] = useState("");

  const currentUserRole = localStorage.getItem("role");

  useEffect(() => {
    if (currentUserRole === "admin") {
      loadUsers();
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  async function loadUsers() {
    try {
      const data = await fetchUsers();
      setUsers(Array.isArray(data) ? data : []);
    } catch {
      setMessage("Erreur lors du chargement des utilisateurs.");
    }
  }

  async function handleCreateUser() {
    if (!username || !password) return;
    const result = await createUser(username, password, newUserRole);
    setMessage(typeof result === "string" ? result : "Utilisateur créé.");
    await loadUsers();
    setUsername("");
    setPassword("");
    setNewUserRole("analyst");
  }

  async function handleDelete(name: string) {
    if (!window.confirm(`Supprimer ${name} ?`)) return;
    const result = await deleteUser(name);
    setMessage(typeof result === "string" ? result : "Utilisateur supprimé.");
    await loadUsers();
  }

  async function handleRoleChange(name: string) {
    const role = window.prompt("Nouveau rôle : admin / analyst / viewer");
    if (!role || !ROLES.includes(role)) return;
    const result = await updateUserRole(name, role);
    setMessage(typeof result === "string" ? result : "Rôle mis à jour.");
    await loadUsers();
  }

  if (currentUserRole !== "admin") {
    return (
      <div className="empty-state">
        <div style={{ fontSize: 18, fontWeight: 600, color: "var(--text)", marginBottom: 6 }}>
          403 — Accès refusé
        </div>
        Cette page est réservée aux administrateurs.
      </div>
    );
  }

  return (
    <div>
      {message && <div className="banner">{message}</div>}

      <div className="panel" style={{ marginBottom: 20 }}>
        <h3 className="panel-title">Créer un utilisateur</h3>
        <div className="toolbar" style={{ marginBottom: 0 }}>
          <input
            className="search-input"
            placeholder="Nom d'utilisateur"
            value={username}
            onChange={(e) => setUsername(e.target.value)}
          />
          <input
            className="search-input"
            type="password"
            placeholder="Mot de passe"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
          />
          <select
            className="filter-select"
            value={newUserRole}
            onChange={(e) => setNewUserRole(e.target.value)}
          >
            <option value="admin">Admin</option>
            <option value="analyst">Analyst</option>
            <option value="viewer">Viewer</option>
          </select>
          <button className="btn btn-primary" onClick={handleCreateUser}>
            + Créer
          </button>
        </div>
      </div>

      <div className="table-wrap">
        <table className="data-table">
          <thead>
            <tr>
              <th>Utilisateur</th>
              <th>Rôle</th>
              <th style={{ width: 200 }}>Actions</th>
            </tr>
          </thead>
          <tbody>
            {users.map((user) => (
              <tr key={user.username}>
                <td className="cell-strong cell-mono">{user.username}</td>
                <td>
                  <span className={`role-badge ${user.role}`}>{user.role}</span>
                </td>
                <td>
                  <button
                    className="btn"
                    style={{ padding: "6px 12px", marginRight: 8 }}
                    onClick={() => handleRoleChange(user.username)}
                  >
                    Modifier rôle
                  </button>
                  <button
                    className="btn btn-ghost"
                    style={{ width: "auto", padding: "6px 12px" }}
                    onClick={() => handleDelete(user.username)}
                  >
                    Supprimer
                  </button>
                </td>
              </tr>
            ))}
            {users.length === 0 && (
              <tr>
                <td colSpan={3}>
                  <div className="empty-state">Aucun utilisateur</div>
                </td>
              </tr>
            )}
          </tbody>
        </table>
      </div>
    </div>
  );
}
