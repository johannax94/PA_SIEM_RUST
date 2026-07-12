import { Link } from "react-router-dom";
import Logo from "../components/Logo";

const PRICE = { monthly: 149, annual: 1490 };

export default function LandingPage() {
  return (
    <div className="landing">
      <header className="landing-topbar">
        <div className="landing-brand">
          <Logo size={32} />
          <span>
            MEDTECH<span className="landing-brand-sub">SIEM</span>
          </span>
        </div>
        <Link to="/login" className="btn btn-primary">
          Login
        </Link>
      </header>

      <main className="landing-hero">
        <div className="landing-logo">
          <Logo size={88} />
        </div>

        <h1 className="landing-title">
          MEDTECH <span className="landing-title-accent">SIEM</span>
        </h1>

        <p className="landing-tagline">
          SIEM pour les PME
        </p>

        <p className="landing-desc">
          Bienvenue sur le SIEM concu spécialement pour les entreprises PME.
          Dans ce SIEM vous trouverez principalement la Collecte centralisée des logs, corrélation par règles et détection des
          menaces en temps réel (brute-force, compromission de comptes, password
          spraying…) — avec tableau de bord, alertes et recherche avancée avec un système de push notification.
        </p>

        <div className="landing-actions">
          <Link to="/login" className="btn btn-primary">
            Accéder à la console
          </Link>
        </div>

        <div className="landing-features">
          <div className="landing-feature">Collecte temps réel</div>
          <div className="landing-feature">règles de détection</div>
          <div className="landing-feature">Tableau de bord &amp; alertes</div>
        </div>
      </main>

      <section className="landing-pricing">
        <h2 className="pricing-title">SIEM accessible aux PME</h2>
        <p className="pricing-sub">Une offre simple, un prix clair.</p>

        <div className="pricing-duo">
          <div className="pricing-card">
            <h3 className="pricing-name">Mensuel</h3>
            <div className="pricing-price">
              <span className="pricing-amount">
                {PRICE.monthly.toLocaleString("fr-FR")} €
              </span>
              <span className="pricing-period">/ mois</span>
            </div>
            <Link
              to="/contact"
              className="btn"
              style={{ width: "100%", justifyContent: "center" }}
            >
              En savoir plus
            </Link>
          </div>

          <div className="pricing-card popular">
            <h3 className="pricing-name">Annuel</h3>
            <div className="pricing-price">
              <span className="pricing-amount">
                {PRICE.annual.toLocaleString("fr-FR")} €
              </span>
              <span className="pricing-period">/ an</span>
            </div>
            <Link
              to="/contact"
              className="btn btn-primary"
              style={{ width: "100%", justifyContent: "center" }}
            >
              En savoir plus
            </Link>
          </div>
        </div>
      </section>

      <footer className="landing-footer">
        © {new Date().getFullYear()} MedTech — Projet SIEM ESGI Meguedad Johanna / Languedoc Clement
      </footer>
    </div>
  );
}
