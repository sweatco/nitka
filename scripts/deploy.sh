#!/bin/bash

set -eox pipefail

source .env
source dev.env

make build

echo ">> Deploying contract: $CONTRACT_NAME"

near dev-deploy --wasmFile "res/$CONTRACT_NAME.wasm" --initFunction "init"
