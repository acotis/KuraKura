/** @type {import('tailwindcss').Config} */
export default {
  content: ["./src/**/*.{js,jsx,ts,tsx}"],
  theme: {
    extend: {
      keyframes: {
        cw: {
          "100%": { transform: "rotate(90deg)" },
        },
      },
      animation: {
        cw: "cw 0.5s ease forwards",
      },
    },
  },
  plugins: [],
};
