
cd wasm
wasm-pack build
cd ..

cd web
npm run build
cd ..

cargo build
