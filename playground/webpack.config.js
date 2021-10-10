const path = require("path");
const zlib = require("zlib");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const { BROTLI_PARAM_QUALITY, BROTLI_MAX_QUALITY } = zlib.constants;

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
  entry: "./src/index.tsx",
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
      {
        test: /\.json?$/,
        type: "asset/resource",
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
      template: path.resolve(__dirname, "src", "index.html"),
    }),
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, "..", "wasm-core"),
      forceMode: "production",
    }),
  ],
  experiments: {
    asyncWebAssembly: true,
  },
};
