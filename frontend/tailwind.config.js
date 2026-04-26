/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        "ui-blue": {
          primary: "#276FD1",
          light: "#EAF1F9",
          lighter: "#F1F8FE",
        },
        "ui-green": {
          DEFAULT: "#059669",
          light: "#E9FDF2",
          lighter: "#EDFCF5",
        },
        "ui-red": {
          DEFAULT: "#dc2626",
          light: "#FCECED",
        },
      },
    },
  },
  plugins: [],
}