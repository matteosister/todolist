const path = require('path');
const {CleanWebpackPlugin} = require('clean-webpack-plugin');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
    mode: 'development',
    entry: './js/index.js',
    plugins: [
        new CleanWebpackPlugin(),
        new HtmlWebpackPlugin({
            title: 'Development',
            template: "js/index.html"
        }),
        new WasmPackPlugin({
            crateDirectory: path.resolve(__dirname, "../"),
            outDir: path.resolve(__dirname, "../dist")
        })
    ],
    output: {
        path: path.resolve(__dirname, '../dist'),
        filename: "todolist.bundle.[hash].js",
        publicPath: "/"
    },
    devServer: {
        contentBase: path.join(__dirname, '../dist'),
        compress: true,
        port: 8000,
        historyApiFallback: true
    }
};