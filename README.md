
# trilier.com server

**Tr**ans**l**ation Ate**lier**

## how to dev

`.env` file which has `TRANSLATE_API_KEY` defintion is required in root directory.
`scripts/install.sh` generates it automatically.

```config
# .env
TRANSLATE_API_KEY = (set key here)
```

Execute scripts files in root dir:

```console
$ # install
$ sh scripts/install.sh

$ # build server / wasm for client / client
$ sh scripts/build.sh

$ # run server
$ sh scripts/run-server.sh
```
