
cd app-wasm
wasm-pack build
cd ..

cd app-web
npm run build
cd ..

cargo build
