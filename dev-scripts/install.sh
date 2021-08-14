
SCRIPTS_DIR=$(dirname "$0")

sh $SCRIPTS_DIR/build.sh

cd app-web
echo ====== `pwd` start ======
npm install
npm audit fix
echo ====== `pwd` end ======
cd ..

DOT_ENV=.env
if ! test -f "$DOT_ENV"; then
    echo ====== `pwd` start ======
    printf "TRANSLATE_API_KEY = (set key here)\n" > "$DOT_ENV"
    echo ====== `pwd` end ======
fi
