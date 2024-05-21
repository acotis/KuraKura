import typography from "@tailwindcss/typography";
import daisyui from "daisyui";

/** @type {import('tailwindcss').Config} */
export default {
  content: ["./src/**/*.{js,jsx,ts,tsx}"],
  theme: {
    extend: {
      colors: {
        primary: "#566CB8",
        secondary: "#F1C1AA",
        accent: "#ED7D9C",
      },
      keyframes: {
        cw: {
          "0%": { transform: "rotate(0deg)" },
          "100%": { transform: "rotate(90deg)" },
        },
      },
      animation: {
        cw: "cw 0.5s ease forwards",
      },
    },
  },
  plugins: [typography, daisyui],
};
