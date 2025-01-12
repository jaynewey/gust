/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.{html,rs}"],
  theme: {
    fontFamily: {
      sans: ["Manrope", "sans-serif"],
    },
    extend: {
      transitionDuration: {
        2000: "5000ms",
      },
    },
  },
  plugins: [],
  variants: {
    extend: {
      display: ["group-focus"],
    },
  },
};
