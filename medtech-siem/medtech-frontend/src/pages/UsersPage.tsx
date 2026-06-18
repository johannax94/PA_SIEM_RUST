import { useEffect, useState } from "react";
import {
  fetchUsers,
  createUser,
  deleteUser,
  updateUserRole,
} from "../services/api";

export default function UsersPage() {

  const [users, setUsers] =
    useState<any[]>([]);

  const [username, setUsername] =
    useState("");

  const [password, setPassword] =
    useState("");

  const [newUserRole, setNewUserRole] =
    useState("analyst");

  const [message, setMessage] =
    useState("");

  const currentUserRole =
    localStorage.getItem("role");

  if (currentUserRole !== "admin") {
    return (
      <div
        style={{
          background: "#0f172a",
          minHeight: "100vh",
          color: "white",
          padding: "40px",
        }}
      >
        <h1>403 - Access Denied</h1>
        <p>
          Cette page est réservée aux administrateurs.
        </p>
      </div>
    );
  }

  useEffect(() => {
    loadUsers();
  }, []);

  async function loadUsers() {
    const data = await fetchUsers();
    setUsers(data);
  }

  async function handleCreateUser() {

    const result =
      await createUser(
        username,
        password,
        newUserRole
      );

    setMessage(result);

    await loadUsers();

    setUsername("");
    setPassword("");
    setNewUserRole("analyst");
  }

  async function handleDelete(
    username: string
  ) {

    if (
      !window.confirm(
        `Supprimer ${username} ?`
      )
    ) {
      return;
    }

    const result =
      await deleteUser(username);

    setMessage(result);

    await loadUsers();
  }

  async function handleRoleChange(
    username: string
  ) {

    const role = prompt(
      "Nouveau rôle : admin / analyst / viewer"
    );

    if (!role) return;

    const result =
      await updateUserRole(
        username,
        role
      );

    setMessage(result);

    await loadUsers();
  }

  function getRoleBadge(role: string) {

    let background = "#374151";

    if (role === "admin") {
      background = "#991b1b";
    }

    if (role === "analyst") {
      background = "#1d4ed8";
    }

    if (role === "viewer") {
      background = "#065f46";
    }

    return (
      <span
        style={{
          background,
          color: "white",
          padding: "4px 12px",
          borderRadius: "999px",
          fontSize: "12px",
          fontWeight: "bold",
          textTransform: "uppercase",
        }}
      >
        {role}
      </span>
    );
  }

  return (
    <div
      style={{
        background: "#0f172a",
        minHeight: "100vh",
        color: "white",
        padding: "30px",
      }}
    >

      <h1
        style={{
          marginBottom: "25px",
          fontSize: "32px",
        }}
      >
        👥 User Management
      </h1>

      {message && (
        <div
          style={{
            marginBottom: "20px",
            padding: "12px",
            borderRadius: "8px",
            background:
              message.includes("created") ||
              message.includes("updated") ||
              message.includes("deleted")
                ? "#14532d"
                : "#7f1d1d",
          }}
        >
          {message}
        </div>
      )}

      {/* CREATE USER CARD */}

      <div
        style={{
          background: "#111827",
          border: "1px solid #1f2937",
          borderRadius: "12px",
          padding: "24px",
          marginBottom: "30px",
        }}
      >
        <h3
          style={{
            marginBottom: "20px",
          }}
        >
          Create User
        </h3>

        <div
          style={{
            display: "flex",
            gap: "12px",
            flexWrap: "wrap",
          }}
        >
          <input
            placeholder="Username"
            value={username}
            onChange={(e) =>
              setUsername(e.target.value)
            }
            style={inputStyle}
          />

          <input
            type="password"
            placeholder="Password"
            value={password}
            onChange={(e) =>
              setPassword(e.target.value)
            }
            style={inputStyle}
          />

          <select
            value={newUserRole}
            onChange={(e) =>
              setNewUserRole(
                e.target.value
              )
            }
            style={inputStyle}
          >
            <option value="admin">
              Admin
            </option>

            <option value="analyst">
              Analyst
            </option>

            <option value="viewer">
              Viewer
            </option>
          </select>

          <button
            onClick={handleCreateUser}
            style={createButton}
          >
            + Create
          </button>
        </div>
      </div>

      {/* USERS TABLE */}

      <div
        style={{
          background: "#111827",
          border: "1px solid #1f2937",
          borderRadius: "12px",
          overflow: "hidden",
        }}
      >
        <table
          style={{
            width: "100%",
            borderCollapse: "collapse",
          }}
        >

          <thead>
            <tr
              style={{
                background: "#0b1220",
              }}
            >
              <th style={thStyle}>
                Username
              </th>

              <th style={thStyle}>
                Role
              </th>

              <th style={thStyle}>
                Actions
              </th>
            </tr>
          </thead>

          <tbody>

            {users.map((user) => (

              <tr
                key={user.username}
                style={{
                  borderBottom:
                    "1px solid #1f2937",
                }}
              >

                <td style={tdStyle}>
                  {user.username}
                </td>

                <td style={tdStyle}>
                  {getRoleBadge(user.role)}
                </td>

                <td style={tdStyle}>

                  <button
                    style={editButton}
                    onClick={() =>
                      handleRoleChange(
                        user.username
                      )
                    }
                  >
                    Edit
                  </button>

                  <button
                    style={deleteButton}
                    onClick={() =>
                      handleDelete(
                        user.username
                      )
                    }
                  >
                    Delete
                  </button>

                </td>

              </tr>

            ))}

          </tbody>

        </table>
      </div>

    </div>
  );
}

const inputStyle = {
  background: "#1f2937",
  border: "1px solid #374151",
  color: "white",
  padding: "10px",
  borderRadius: "8px",
};

const thStyle = {
  textAlign: "left" as const,
  padding: "16px",
  color: "#cbd5e1",
};

const tdStyle = {
  padding: "16px",
};

const createButton = {
  background: "#06b6d4",
  color: "white",
  border: "none",
  borderRadius: "8px",
  padding: "10px 18px",
  cursor: "pointer",
  fontWeight: "bold",
};

const editButton = {
  background: "#2563eb",
  color: "white",
  border: "none",
  borderRadius: "8px",
  padding: "8px 12px",
  cursor: "pointer",
  marginRight: "8px",
};

const deleteButton = {
  background: "#dc2626",
  color: "white",
  border: "none",
  borderRadius: "8px",
  padding: "8px 12px",
  cursor: "pointer",
};