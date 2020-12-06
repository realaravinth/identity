const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');

module.exports = {
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  mode: "development",
  plugins: [
    new CopyWebpackPlugin(['index.html']),
    new CopyWebpackPlugin(['signup.html']),
    new CopyWebpackPlugin(['signin.html']),
    new CopyWebpackPlugin(['main.css']),
    new CopyWebpackPlugin(['./favicon/favicon.ico'])
  ],
};
