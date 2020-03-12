/* eslint-disable @typescript-eslint/no-var-requires */
const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const webpack = require("webpack");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const ASSET_PATH = process.env.ASSET_PATH || "";

module.exports = {
  resolve: {
    extensions: [".ts", ".tsx", ".js", ".jsx", ".wasm"]
  },
  output: {
    globalObject: "this",
    publicPath: ASSET_PATH
  },
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        loader: "ts-loader",
        options: {
          transpileOnly: true
        }
      },
      {
        test: /\.(jpg|png)$/,
        loader: "file-loader?name=[name].[ext]"
      }
    ]
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: path.join(__dirname, "src/index.html")
    }),
    new WasmPackPlugin({
      crateDirectory: path.join(__dirname, "tora-breakout-wasm"),
      outName: "tora_breakout_wasm",
      withTypeScript: true
    }),
    new webpack.ProvidePlugin({
      TextDecoder: ["text-encoding", "TextDecoder"],
      TextEncoder: ["text-encoding", "TextEncoder"]
    }),
    new webpack.DefinePlugin({
      "process.env.ASSET_PATH": JSON.stringify(ASSET_PATH),
      ASSET_URL: JSON.stringify(ASSET_PATH)
    })
  ]
};
