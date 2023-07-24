/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.{html,rs}"],
  theme: {
    fontFamily: {
      sans: ["Manrope", "sans-serif"],
    },
    extend: {},
  },
  plugins: [],
  variants: {
    extend: {
      display: ["group-focus"],
    },
  },
};
