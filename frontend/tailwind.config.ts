import type { Config } from "tailwindcss";

export default {
  content: ["./index.html", "./src/**/*.{ts,tsx}"],
  darkMode: "class",
  theme: {
    extend: {
      colors: {
        cream: {
          50: "#fdf8f3",
          100: "#faf0e4",
          200: "#f5e6d3",
          300: "#edd9c2",
          400: "#d4b896",
          500: "#c4a37c",
        },
        espresso: {
          50: "#f0ede8",
          100: "#d4cec4",
          200: "#b8ad9c",
          300: "#9c8d74",
          400: "#8b7355",
          500: "#6b5d4f",
          600: "#4a3f35",
          700: "#2c2416",
          800: "#1a1510",
          900: "#0d0a08",
        },
        terra: {
          400: "#e57a77",
          500: "#d97642",
          600: "#c15e2e",
        },
        harvest: {
          300: "#d4a574",
          500: "#4a7c59",
          600: "#3d6a4b",
        },
        sky: {
          400: "#7d9ba8",
        },
      },
      fontFamily: {
        sans: ["Jost", "Century Gothic", "Futura", "Avenir", "system-ui", "sans-serif"],
        mono: ["SF Mono", "JetBrains Mono", "Menlo", "monospace"],
      },
      borderRadius: {
        mcm: "12px",
        "mcm-lg": "16px",
        "mcm-xl": "20px",
      },
      boxShadow: {
        mcm: "0 2px 12px rgba(44,36,22,0.06)",
        "mcm-md": "0 4px 20px rgba(44,36,22,0.08)",
        "mcm-lg": "0 8px 32px rgba(44,36,22,0.10)",
      },
    },
  },
  plugins: [],
} satisfies Config;
