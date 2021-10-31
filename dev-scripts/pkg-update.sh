
cd app-wasm
echo ====== `pwd` start ======
cargo update
cargo build
wasm-pack build
echo ====== `pwd` end ======
cd ..

cd app-web
echo ====== `pwd` start ======
npm upgrade
npm update
npm run build
echo ====== `pwd` end ======
cd ..

echo ====== `pwd` start ======
cargo update
cargo build
echo ====== `pwd` end ======
