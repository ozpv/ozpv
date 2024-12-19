/** @type {import('tailwindcss').Config} */
export default {
  content: {
    files: ["./src/**/*.rs"],
    transform: {
      rs: (content) => content.replace(/(?:^|\s)class:/g, ' '),
    },
  },
  theme: {
  },
  plugins: [],
}
