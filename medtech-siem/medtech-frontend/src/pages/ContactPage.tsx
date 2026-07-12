import { useState } from "react";
import type { FormEvent } from "react";
import { Link } from "react-router-dom";
import Logo from "../components/Logo";
import { sendContactMessage } from "../services/api";

// L'email doit contenir un @ et un domaine.
const EMAIL_RE = /^[^@\s]+@[^@\s]+\.[^@\s]+$/;
// Le commentaire : uniquement lettres (accentuées incluses), chiffres,
// espaces et ponctuation simple . , ! ? — aucun caractère spécial.
const COMMENT_RE = /^[\p{L}\p{N}\s.,!?]*$/u;

export default function ContactPage() {
  const [email, setEmail] = useState("");
  const [message, setMessage] = useState("");
  const [status, setStatus] = useState<"idle" | "sending" | "sent" | "error">(
    "idle"
  );
  const [validationError, setValidationError] = useState("");

  async function handleSubmit(e: FormEvent) {
    e.preventDefault();
    if (status === "sending") return;

    // Validation d'entrée (l'email doit avoir un @, le commentaire pas de
    // caractères spéciaux) — bloque avant tout envoi.
    if (!EMAIL_RE.test(email.trim())) {
      setValidationError("Veuillez saisir une adresse email valide (elle doit contenir un @).");
      return;
    }
    if (!COMMENT_RE.test(message)) {
      setValidationError(
        "Le commentaire ne doit pas contenir de caractères spéciaux (lettres, chiffres, espaces et . , ! ? uniquement)."
      );
      return;
    }
    setValidationError("");
    setStatus("sending");

    try {
      await sendContactMessage(email, message);
      setStatus("sent");
      setEmail("");
      setMessage("");
    } catch {
      setStatus("error");
    }
  }

  return (
    <div className="landing">
      <header className="landing-topbar">
        <div className="landing-brand">
          <Logo size={32} />
          <span>
            MEDTECH<span className="landing-brand-sub">SIEM</span>
          </span>
        </div>
        <div style={{ display: "flex", gap: 10 }}>
          <Link to="/" className="btn">
            Accueil
          </Link>
          <Link to="/login" className="btn btn-primary">
            Login
          </Link>
        </div>
      </header>

      <main className="contact-main">
        <h1 className="contact-title">
          En savoir <span className="landing-title-accent">plus</span>
        </h1>
        <p className="contact-sub">
          Découvrez notre solution et contactez-nous pour toute question.
        </p>

        <div className="contact-grid">
          <section className="contact-about panel">
            <h2 className="contact-panel-title">Notre SIEM</h2>
            <p>
              <strong>MedTech SIEM</strong> est une solution de{" "}
              <em>Security Information &amp; Event Management</em> créée en{" "}
              <strong>2025</strong> dans le cadre du projet annuel ESGI, par
              Meguedad Johanna et Languedoc Clement.
            </p>
            <p>
              Pensée pour les infrastructures médicales et les PME, elle a pour
              objectifs :
            </p>
            <ul className="contact-purposes">
              <li>
                Centraliser la collecte des logs de l'ensemble de
                l'infrastructure en temps réel
              </li>
              <li>
                Détecter les menaces grâce à des règles de corrélation
                (brute-force, password spraying, compromission de comptes,
                scans réseau…) alignées sur MITRE ATT&amp;CK
              </li>
              <li>
                Alerter les équipes et offrir un tableau de bord clair avec
                recherche avancée
              </li>
              <li>
                Rendre la cybersécurité accessible aux PME avec une offre
                simple et un prix transparent
              </li>
            </ul>
            <p className="contact-about-note">
              Backend haute performance en Rust, interface web moderne, et
              détection en continu.
            </p>
          </section>

          <section className="contact-form-panel panel">
            <h2 className="contact-panel-title">Nous contacter</h2>

            {status === "sent" ? (
              <div className="contact-success">
                <div className="contact-success-icon">✓</div>
                <p>
                  Merci ! Votre message a bien été envoyé. Nous vous répondrons
                  au plus vite.
                </p>
                <button className="btn" onClick={() => setStatus("idle")}>
                  Envoyer un autre message
                </button>
              </div>
            ) : (
              <form onSubmit={handleSubmit}>
                <div className="login-field">
                  <label htmlFor="contact-email">Votre email</label>
                  <input
                    id="contact-email"
                    type="email"
                    required
                    placeholder="vous@entreprise.fr"
                    value={email}
                    onChange={(e) => setEmail(e.target.value)}
                  />
                </div>

                <div className="login-field">
                  <label htmlFor="contact-message">Commentaire</label>
                  <textarea
                    id="contact-message"
                    className="contact-textarea"
                    required
                    rows={7}
                    placeholder="Votre question, demande de démo, remarque…"
                    value={message}
                    onChange={(e) => setMessage(e.target.value)}
                  />
                </div>

                {validationError && (
                  <p className="login-error">{validationError}</p>
                )}

                {status === "error" && (
                  <p className="login-error">
                    L'envoi a échoué. Veuillez réessayer ou nous écrire
                    directement à medtechsiem@gmail.com.
                  </p>
                )}

                <button
                  type="submit"
                  className="btn btn-primary"
                  style={{ width: "100%", justifyContent: "center" }}
                  disabled={status === "sending"}
                >
                  {status === "sending" ? "Envoi en cours…" : "Envoyer"}
                </button>
              </form>
            )}
          </section>
        </div>
      </main>

      <footer className="landing-footer">
        © {new Date().getFullYear()} MedTech — Projet SIEM ESGI Meguedad
        Johanna / Languedoc Clement
      </footer>
    </div>
  );
}
