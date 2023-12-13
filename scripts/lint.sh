#!/bin/bash
set -eox pipefail

rustup component add clippy

cargo clippy --all \
  -- \
  \
  -W clippy::all \
  -W clippy::pedantic \
  \
  -A clippy::module_name_repetitions \
  -A clippy::needless-pass-by-value \
  -A clippy::missing-errors-doc \
  -A clippy::module-inception \
  -A clippy::missing-panics-doc \
  -A clippy::must-use-candidate \
  -A clippy::return-self-not-must-use \
  -A clippy::new-ret-no-self \
  -A clippy::wrong-self-convention \
  \
  -D warnings
