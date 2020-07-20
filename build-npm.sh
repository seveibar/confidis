rm -rf ./pkg
mkdir ./pkg
wasm-pack build -t nodejs -d ./pkg/node
wasm-pack build -t bundler -d ./pkg/webpack
cp ./package.json ./pkg/package.json
touch ./pkg/.npmignore
