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
import ProtectedRoute from "./components/ProtectedRoute";
import Logo from "./components/Logo";

const PAGE_TITLES: Record<string, string> = {
  "/": "Dashboard",
  "/alerts": "Alertes",
  "/logs": "Logs",
};

function logout() {
  localStorage.removeItem("token");
  window.location.href = "/login";
}

function Shell() {
  const location = useLocation();

  if (location.pathname === "/login") {
    return <LoginPage />;
  }

  return (
    <div className="app-shell">
      <aside className="sidebar">
        <div className="sidebar-logo">
          <Logo />
        </div>

        <nav>
          <NavLink to="/" end className={({ isActive }) => `nav-link${isActive ? " active" : ""}`}>
            Dashboard
          </NavLink>
          <NavLink to="/alerts" className={({ isActive }) => `nav-link${isActive ? " active" : ""}`}>
            Alertes
          </NavLink>
          <NavLink to="/logs" className={({ isActive }) => `nav-link${isActive ? " active" : ""}`}>
            Logs
          </NavLink>
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
            Live · rafraîchissement auto 10 s
          </div>
        </header>

        <div className="content">
          <Routes>
            <Route
              path="/"
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
            <Route path="*" element={<Navigate to="/" replace />} />
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
