"use strict";

const path = require('path');
const CopyPlugin = require('copy-webpack-plugin');
const HtmlPlugin = require('html-webpack-plugin');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const TerserJSPlugin = require('terser-webpack-plugin');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');
const OptimizeCSSAssetsPlugin = require('optimize-css-assets-webpack-plugin');

function absPath(...pathSegments) {
  return path.resolve(__dirname, ...pathSegments);
}

module.exports = (env, argv) => {
  const isProd = argv.mode === 'production';
  const cargo_args = isProd ? '--features console_error_panic_hook' : '';

  return {
    entry: {
      main: './web/index.js',
    },
    output: {
      path: absPath('dist'),
      filename: '[name].bundle.js',
      webassemblyModuleFilename: 'app.wasm',
      publicPath: '/',
    },
    optimization: {
      minimize: isProd,
      minimizer: [new TerserJSPlugin({}), new OptimizeCSSAssetsPlugin({})],
    },
    devServer: {
      contentBase: absPath('dist'),
      compress: isProd,
      port: 8000,
      hot: !isProd,
      historyApiFallback: true,
    },
    devtool: 'source-map',
    module: {
      rules: [
        {
          test: [/.css$|.s[ac]ss$/],
          use: [
            isProd ?
              { loader: MiniCssExtractPlugin.loader, options: { hmr: !isProd } }
              : 'style-loader',
            { loader: 'css-loader', options: { sourceMap: !isProd } },
            { loader: 'sass-loader', options: { sourceMap: !isProd } }
          ]
        },
      ],
    },
    plugins: [
      new WasmPackPlugin({
        crateDirectory: absPath('.'),
        extraArgs: '--no-typescript -- ' + cargo_args,
      }),
      new HtmlPlugin({
        template: './web/index.html',
        //favicon: './web/static/favicon.ico',
      }),
      new MiniCssExtractPlugin({
        filename: '[name].css',
        chunkFilename: '[id].css',
      }),
      new CopyPlugin([
        absPath('web', 'static'),
      ]),
    ],
    watch: !isProd,
  };
};
