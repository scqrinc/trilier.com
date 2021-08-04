
SCRIPTS_DIR=$(dirname "$0")

sh $SCRIPTS_DIR/build.sh

cd web
npm install
npm audit fix
cd ..
