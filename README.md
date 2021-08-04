
# trilier.com server

**Tr**anslation Ate**lier**

## how to dev

add `.env` file to the root directory and set `TRANSLATE_API_KEY` in it:

```config
# /.env
TRANSLATE_API_KEY = (set key here)
```

execute scripts files in root dir

```console
$ # install
$ sh scripts/install.sh

$ # build server / wasm for client / client
$ sh scripts/build.sh

$ # run server
$ sh scripts/run-server.sh
```
