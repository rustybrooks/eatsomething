const { merge } = require('webpack-merge');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const path = require('path');
const baseConfig = require('./webpack.config.base');
// const UglifyJsPlugin = require('uglifyjs-webpack-plugin');

module.exports = merge(baseConfig, {
  mode: 'production',
  plugins: [
    new HtmlWebpackPlugin({
      title: 'Caching',
      filename: 'index.html',
      hash: true,
      template: 'src/index.html',
    }),
    // Minify JS
    // new UglifyJsPlugin({
    //  sourceMap: false,
    //  // compress: true,
    // }),
    // Minify CSS
    // new webpack.LoaderOptionsPlugin({
    //  minimize: true,
    // }),
  ],
  optimization: {
    runtimeChunk: 'single',
    moduleIds: 'deterministic',
    splitChunks: {
      cacheGroups: {
        vendor: {
          test: /[\\/]node_modules[\\/]/,
          name: 'vendors',
          chunks: 'all',
        },
      },
    },
  },
  output: {
    filename: '[name].[contenthash].js',
    path: path.resolve(__dirname, 'dist'),
    clean: true,
    publicPath: '/dist',
  },
});
