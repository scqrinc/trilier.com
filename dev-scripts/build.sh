
cd app-wasm
echo ====== `pwd` start ======
wasm-pack build
echo ====== `pwd` end ======
cd ..

cd app-web
echo ====== `pwd` start ======
npm run build
echo ====== `pwd` end ======
cd ..

echo ====== `pwd` start ======
cargo build
echo ====== `pwd` end ======
