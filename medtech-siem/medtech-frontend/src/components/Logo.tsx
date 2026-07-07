/* Logo MedTech SIEM : ligne de pouls (ECG) sur fond dégradé */
export default function Logo({ size = 34 }: { size?: number }) {
  return (
    <svg width={size} height={size} viewBox="0 0 40 40">
      <defs>
        <linearGradient id="logoGrad" x1="0" y1="0" x2="1" y2="1">
          <stop offset="0%" stopColor="#4d7ea8" />
          <stop offset="100%" stopColor="#6f9cc4" />
        </linearGradient>
      </defs>
      <rect width="40" height="40" rx="10" fill="url(#logoGrad)" />
      <polyline
        points="7,21 14,21 17,12 21,28 24,17 26,21 33,21"
        fill="none"
        stroke="white"
        strokeWidth="2.4"
        strokeLinecap="round"
        strokeLinejoin="round"
      />
    </svg>
  );
}
