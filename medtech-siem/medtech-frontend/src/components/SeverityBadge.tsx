const KNOWN = ["critical", "high", "medium", "low", "info"];

export function normalizeSeverity(severity?: string): string {
  const s = (severity ?? "info").toLowerCase();
  if (KNOWN.includes(s)) return s;
  if (s === "warning" || s === "warn") return "medium";
  if (s === "error") return "high";
  return "info";
}

export default function SeverityBadge({ severity }: { severity?: string }) {
  const level = normalizeSeverity(severity);
  return <span className={`badge ${level}`}>{severity ?? "info"}</span>;
}
