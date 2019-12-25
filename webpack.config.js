const path = require("path");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const { CleanWebpackPlugin } = require("clean-webpack-plugin");
const dist = path.resolve(__dirname, "../pkg");

module.exports = {
    mode: "development",
    entry: './js/main',
    output: {
        filename: 'main.js',
        path: dist,
    },
    devServer: {
        contentBase: dist,
        // You can connect to dev server from devices in your network (e.g. 192.168.0.3:8000).
        host: "0.0.0.0",
        port: 8000,
        noInfo: true,
        stats: "errors-only",
        overlay: {
            warnings: true,
            errors: true
        },
        historyApiFallback: true,
    },
    plugins: [
        new CleanWebpackPlugin(),
        new WasmPackPlugin({
            crateDirectory: __dirname,
            outDir: path.resolve(__dirname, "pkg")
        }),
    ]
};