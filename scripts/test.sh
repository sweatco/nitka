#!/bin/bash
set -eox pipefail

source .env

cargo test --package $CONTRACT_NAME
