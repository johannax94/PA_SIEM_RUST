import {
  BrowserRouter,
  Routes,
  Route,
  NavLink,
  Navigate,
  useLocation,
} from "react-router-dom";
import DashboardPage from "./pages/DashboardPage";
import AlertsPage from "./pages/AlertsPage";
import LogsPage from "./pages/LogsPage";
import LoginPage from "./pages/LoginPage";
import UsersPage from "./pages/UsersPage";
import LandingPage from "./pages/LandingPage";
import ProtectedRoute from "./components/ProtectedRoute";
import Logo from "./components/Logo";

const PAGE_TITLES: Record<string, string> = {
  "/dashboard": "Dashboard",
  "/alerts": "Alertes",
  "/logs": "Logs",
  "/users": "Gestion des utilisateurs",
};

function logout() {
  localStorage.removeItem("token");
  localStorage.removeItem("role");
  window.location.href = "/login";
}

function Shell() {
  const location = useLocation();
  const role = localStorage.getItem("role");

  if (location.pathname === "/") {
    return <LandingPage />;
  }

  if (location.pathname === "/login") {
    return <LoginPage />;
  }

  const navClass = ({ isActive }: { isActive: boolean }) =>
    `nav-link${isActive ? " active" : ""}`;

  return (
    <div className="app-shell">
      <aside className="sidebar">
        <div className="sidebar-logo">
          <Logo />
          <div>
            MEDTECH
            <span className="logo-sub">SIEM</span>
          </div>
        </div>

        <nav>
          <NavLink to="/dashboard" className={navClass}>
            Dashboard
          </NavLink>
          <NavLink to="/alerts" className={navClass}>
            Alertes
          </NavLink>
          <NavLink to="/logs" className={navClass}>
            Logs
          </NavLink>
          {role === "admin" && (
            <NavLink to="/users" className={navClass}>
              Utilisateurs
            </NavLink>
          )}
        </nav>

        <div className="sidebar-footer">
          <button className="btn btn-ghost" onClick={logout}>
            Logout
          </button>
        </div>
      </aside>

      <div className="main">
        <header className="topbar">
          <h1>{PAGE_TITLES[location.pathname] ?? "Dashboard"}</h1>
          <div className="live-indicator">
            <span className="pulse-dot" />
            Live
          </div>
        </header>

        <div className="content">
          <Routes>
            <Route
              path="/dashboard"
              element={
                <ProtectedRoute>
                  <DashboardPage />
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
              path="/logs"
              element={
                <ProtectedRoute>
                  <LogsPage />
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
            <Route path="*" element={<Navigate to="/dashboard" replace />} />
          </Routes>
        </div>
      </div>
    </div>
  );
}

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="*" element={<Shell />} />
      </Routes>
    </BrowserRouter>
  );
}

export default App;
