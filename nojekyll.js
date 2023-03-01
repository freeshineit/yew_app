const rimraf = require("rimraf");
const shell = require("shelljs");
const path = require("path");
const fs = require("fs");

const start = async () => {
  try {
    await createDotNojekyll();
  } catch (error) {
    shell.exit(1);
  }
};

/**
 * .nojekyll 就是告诉 Github Pages 当前网站不是基于Jekyll构建的，不要忽略掉下划线开头的文件和文件夹
 */
async function createDotNojekyll() {
  const dotNojekyll = path.resolve(__dirname, "./docs/.nojekyll");

  return new Promise((resolve, reject) => {
    fs.stat(dotNojekyll, (err, stat) => {
      if (err === null) {
      } else if (err.code == "ENOENT") {
        fs.writeFile(dotNojekyll, "", "utf8", function (error) {
          if (error) {
            reject(error);
            return false;
          }
          resolve();
        });
      } else {
        reject(err);
      }
    });
  });
}

start();
