const path = require('path');
const CopyWebpackPlugin = require('copy-webpack-plugin')

module.exports = {
    mode: "development",
    entry: "./js/index.js",
    devtool: 'source-map',
    output: {
        path: path.resolve(__dirname, "dist"),
        filename: "bundle.js",
    },
    plugins: [
        new CopyWebpackPlugin([{ from: 'static_web' }]),
    ],
};
