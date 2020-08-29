const path = require('path');
const {CleanWebpackPlugin} = require('clean-webpack-plugin');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
    mode: 'production',
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
    module: {
        rules: [
            {
                test: /\.css$/i,
                use: ['style-loader', 'css-loader'],
            },
        ],
    },
    resolve: {
        alias: {
            css: path.resolve(__dirname, '../css/'),
        }
    },
    output: {
        path: path.resolve(__dirname, '../dist'),
        filename: 'todolist.bundle.js',
        publicPath: "/todolist/"
    }
};
