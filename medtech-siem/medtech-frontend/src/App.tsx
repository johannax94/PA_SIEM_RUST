import { BrowserRouter, Routes, Route, Link, useLocation } from "react-router-dom";

import LandingPage from "./pages/LandingPage";
import LoginPage from "./pages/LoginPage";
import DashboardPage from "./pages/DashboardPage";
import LogsPage from "./pages/LogsPage";
import AlertsPage from "./pages/AlertsPage";
import UsersPage from "./pages/UsersPage";

import ProtectedRoute from "./components/ProtectedRoute";

function Sidebar() {

  const role = localStorage.getItem("role");

  function logout() {
    localStorage.removeItem("token");
    localStorage.removeItem("role");
    window.location.href = "/";
  }

  return (
    <div
      style={{
        width: 220,
        height: "100vh",
        background: "#111827",
        color: "white",
        padding: 20,
        boxSizing: "border-box",
      }}
    >
      <h2
        style={{
          marginBottom: 30,
        }}
      >
        MedTech SIEM
      </h2>

      <nav>

        <p>
          <Link
            to="/dashboard"
            style={linkStyle}
          >
            Dashboard
          </Link>
        </p>

        <p>
          <Link
            to="/logs"
            style={linkStyle}
          >
            Logs
          </Link>
        </p>

        <p>
          <Link
            to="/alerts"
            style={linkStyle}
          >
            Alertes
          </Link>
        </p>

        {role === "admin" && (

          <p>
            <Link
              to="/users"
              style={linkStyle}
            >
              Utilisateurs
            </Link>
          </p>

        )}

      </nav>

      <button
        onClick={logout}
        style={{
          marginTop: 40,
          width: "100%",
          padding: 12,
          background: "#dc2626",
          color: "white",
          border: "none",
          borderRadius: 8,
          cursor: "pointer",
          fontWeight: "bold",
        }}
      >
        Logout
      </button>

    </div>
  );
}

function AppContent() {

  const location = useLocation();

  const showSidebar =
    location.pathname.startsWith("/dashboard") ||
    location.pathname.startsWith("/logs") ||
    location.pathname.startsWith("/alerts") ||
    location.pathname.startsWith("/users");

  return (

    <div
      style={{
        display: "flex",
        minHeight: "100vh",
      }}
    >

      {showSidebar && <Sidebar />}

      <div
        style={{
          flex: 1,
        }}
      >

        <Routes>

          {/* -------- Site public -------- */}

          <Route
            path="/"
            element={<LandingPage />}
          />

          <Route
            path="/login"
            element={<LoginPage />}
          />

          {/* -------- Console -------- */}

          <Route
            path="/dashboard"
            element={
              <ProtectedRoute>
                <DashboardPage />
              </ProtectedRoute>
            }
          />

          <Route
            path="/logs"
            element={
              <ProtectedRoute>
                <LogsPage />
              </ProtectedRoute>
            }
          />

          <Route
            path="/alerts"
            element={
              <ProtectedRoute>
                <AlertsPage />
              </ProtectedRoute>
            }
          />

          <Route
            path="/users"
            element={
              <ProtectedRoute>
                <UsersPage />
              </ProtectedRoute>
            }
          />

        </Routes>

      </div>

    </div>

  );

}

const linkStyle = {
  color: "white",
  textDecoration: "none",
};

export default function App() {

  return (

    <BrowserRouter>

      <AppContent />

    </BrowserRouter>

  );

}