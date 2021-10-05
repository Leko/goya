const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const swcOption = {
  jsc: {
    parser: {
      syntax: "typescript",
      tsx: true,
      dynamicImport: true,
    },
    target: "es2020",
  },
};

module.exports = {
  entry: "./web/index.tsx",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "[name].[contenthash].js",
  },
  resolve: {
    extensions: [".tsx", ".ts", ".js"],
  },
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: {
          loader: "swc-loader",
          options: swcOption,
        },
      },
      // It's for Viz.js
      {
        test: /\.render\.js$/,
        use: ["file-loader"],
      },
    ],
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: path.resolve(__dirname, "web", "index.html"),
    }),
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, "."),
      forceMode: "production",
    }),
  ],
  experiments: {
    asyncWebAssembly: true,
  },
};
