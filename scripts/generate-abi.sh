#!/bin/bash
set -eox pipefail

source .env

cargo install cargo-near --version 0.4.1

cd $CONTRACT_NAME

CONTRACT_NAME=${CONTRACT_NAME//-/_}

cargo near abi

cd ..

cp ./target/near/${CONTRACT_NAME}/${CONTRACT_NAME}_abi.json ./res/${CONTRACT_NAME}_abi.json

echo Generate ABI: OK
