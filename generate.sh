#!/bin/bash

set -ex

me=$(dirname "$0")
cd "$me"

python3 ./gir/generator.py --strip-docs --embed-docs --gir-files-directories ./gir-files.gtk/ ./gir-files.xfce/ "$@"
