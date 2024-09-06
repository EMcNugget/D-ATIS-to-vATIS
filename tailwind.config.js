/** @type {import('tailwindcss').Config} */
import daisyui from "daisyui";

export default {
  content: ["./index.html", "./src/**/*.{vue,js,ts,jsx,tsx}"],
  theme: {
    extend: {
      colors: {
        outline: "#2b3440",
      },
    },
  },
  plugins: [daisyui],
  daisyui: {
    themes: [
      {
        light: {
          ...require("daisyui/src/theming/themes")["light"],
          primary: "#2b3440",
          accent: "#d2d2d2",
        },
        dark: {
          ...require("daisyui/src/theming/themes")["dark"],
          primary: "#111419",
          accent: "#2a323c",
        },
      },
      "dark",
    ],
  },
};
