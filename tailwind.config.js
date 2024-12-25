module.exports = {
  purge: {
    mode: "all",
    content: [
      "./src/**/*.rs",
      "./index.html",
      "./src/**/*.html",
      "./src/**/*.css",
    ],
  },
  theme: {
    extend: {
      colors: {
        "mygrey-1": "#222831",
      },
    },
  },
  variants: {},
  plugins: [],
};
