import { BrowserRouter, Routes, Route, Link } from "react-router-dom";
import AlertsPage from "./pages/AlertsPage";
import LogsPage from "./pages/LogsPage";

function App() {
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
            <Route path="/logs" element={<LogsPage />} />
          </Routes>
        </div>
      </div>
    </BrowserRouter>
  );
}

export default App;