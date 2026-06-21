import { Link } from "react-router-dom";

export default function LandingPage() {

  return (

    <div
      style={{
        minHeight: "100vh",
        background:
          "linear-gradient(135deg,#020617,#0f172a,#111827)",
        color: "white",
        display: "flex",
        justifyContent: "center",
        alignItems: "center",
        fontFamily: "Arial, sans-serif",
      }}
    >

      <div
        style={{
          maxWidth: 900,
          textAlign: "center",
          padding: 40,
        }}
      >

        <h1
          style={{
            fontSize: 64,
            marginBottom: 10,
          }}
        >
          MedTech SIEM
        </h1>

        <p
          style={{
            color: "#94a3b8",
            fontSize: 24,
            marginBottom: 50,
          }}
        >
          Enterprise Security Monitoring Platform
        </p>

        <div
          style={{
            display: "grid",
            gridTemplateColumns: "repeat(3,1fr)",
            gap: 20,
            marginBottom: 60,
          }}
        >

          <Feature
            title="Real-time Detection"
            text="Detect attacks in real time with customizable detection rules."
          />

          <Feature
            title="SOC Dashboard"
            text="Centralized monitoring of alerts, logs and incidents."
          />

          <Feature
            title="Role Based Access"
            text="Secure administration with RBAC and JWT authentication."
          />

          <Feature
            title="Threat Hunting"
            text="Fast full-text search across millions of logs."
          />

          <Feature
            title="Multi Vendor"
            text="Windows, Linux, Cisco, Fortinet, VMware and more."
          />

          <Feature
            title="Built in Rust"
            text="High-performance backend designed for scalability."
          />

        </div>

        <Link
          to="/login"
          style={{
            padding: "18px 50px",
            background: "#2563eb",
            color: "white",
            textDecoration: "none",
            borderRadius: 10,
            fontWeight: "bold",
            fontSize: 20,
          }}
        >
          Access Console →
        </Link>

      </div>

    </div>

  );

}

function Feature(props: any) {

  return (

    <div
      style={{
        background: "#111827",
        border: "1px solid #334155",
        borderRadius: 12,
        padding: 25,
      }}
    >

      <h3
        style={{
          marginBottom: 10,
          color: "#60a5fa",
        }}
      >
        {props.title}
      </h3>

      <p
        style={{
          color: "#cbd5e1",
          lineHeight: 1.6,
        }}
      >
        {props.text}
      </p>

    </div>

  );

}