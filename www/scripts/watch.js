process.env.NODE_ENV = "development";

const { exec } = require("child_process");

const fs = require("fs-extra");
const paths = require("react-scripts/config/paths");
const webpack = require("webpack");
const webpackconfig = require("react-scripts/config/webpack.config.js");

const config = webpackconfig("development");

// removes react-dev-utils/webpackHotDevClient.js at first in the array
config.entry.shift();

config.mode = "development";
config.devtool = "eval-cheap-module-source-map";
delete config.optimization;

webpack(config).watch({}, (err, stats) => {
  console.error("Watching the files...");
  if (err) {
    console.error(err);
  } else {
    copyPublicFolder();
  }

  console.error(
    stats.toString({
      chunks: false,
      colors: true,
    })
  );
  exec("rm -rf build && mv dist/ build/");
});

function copyPublicFolder() {
  fs.copySync(paths.appPublic, paths.appBuild, {
    dereference: true,
    filter: (file) => file !== paths.appHtml,
  });
}
