import { BrowserRouter, Routes, Route, Link } from "react-router-dom";
import AlertsPage from "./pages/AlertsPage";
import LogsPage from "./pages/LogsPage";
import LoginPage from "./pages/LoginPage";
import ProtectedRoute from "./components/ProtectedRoute";

function App() {
  
  function logout() {
  localStorage.removeItem("token");
  window.location.href = "/login";
}
  return (
    <BrowserRouter>
      <div style={{ display: "flex" }}>
        
        <div
          style={{
            width: "200px",
            height: "100vh",
            background: "#111",
            color: "white",
            padding: "20px",
          }}
        >
          <h2>MedTech SIEM</h2>
          <button onClick={logout}>
            Logout
          </button>
          <nav>
            <p>
              <Link to="/" style={{ color: "white" }}>
                Alertes
              </Link>
            </p>

            <p>
              <Link to="/logs" style={{ color: "white" }}>
                Logs
              </Link>
            </p>
          </nav>
        </div>

        <div style={{ flex: 1, padding: "20px" }}>
          <Routes>
            <Route path="/" element={<AlertsPage />} />
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
            <Route path="/login" element={<LoginPage />} />
          </Routes>
        </div>
      </div>
    </BrowserRouter>
  );
}

export default App;