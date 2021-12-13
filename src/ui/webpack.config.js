const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');

module.exports = {
  mode: "development",
  devtool: 'inline-source-map',
  entry: './src/index.tsx',
  module: {
    rules: [
      {
        test: /\.js$/,
        exclude: /node_modules/,
        use: {
          loader: "babel-loader"
        }
      },      {
        test: /\.tsx?$/,
        use: 'ts-loader',
        exclude: /node_modules/,
      },
    ],
  },
  resolve: {
    extensions: ['.tsx', '.ts', '.js'],
  },
  output: {
    filename: 'bundle.js',
    path: path.resolve(__dirname, 'dist'),
  },
   plugins: [
     new HtmlWebpackPlugin({
       title: 'Hot Module Replacement',
       filename: "index.html",
       hash: true,
       template: "src/index.html",
     }),
   ],
   devServer: {
     // static: './public',
    hot: true,
   },
};
