/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./index.html", "./src/**/*.rs"],
  theme: {
    extend: {
      colors:{
        gradient_start:"#0F0C29",
        gradient_middle:"#302B63",
        gradient_end:"#24243E",
      }
    },
  },
  plugins: [],
}
