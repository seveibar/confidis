rm -rf ./pkg
mkdir ./pkg
wasm-pack build -t nodejs -d ./pkg/node
wasm-pack build -t web -d ./pkg/web
wasm-pack build -t bundler -d ./pkg/bundler
wasm-pack build -t no-modules -d ./pkg/no-modules
cp ./package.json ./pkg/package.json
touch ./pkg/.npmignore
