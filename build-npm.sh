rm -rf ./pkg
mkdir ./pkg
wasm-pack build -t nodejs -d ./pkg/node
wasm-pack build -t bundler -d ./pkg/webpack
# TODO need to fix ./pkg/webpack/package.json "files" to include "confidis_bg.js"
cp ./package.json ./pkg/package.json
touch ./pkg/.npmignore
