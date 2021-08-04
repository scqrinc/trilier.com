
SCRIPTS_DIR=$(dirname "$0")

sh $SCRIPTS_DIR/build.sh

cd web
npm install
npm audit fix
cd ..

DOT_ENV=.env
if ! test -f "$DOT_ENV"; then
    printf "TRANSLATE_API_KEY = (set key here)\n" > "$DOT_ENV"
fi
