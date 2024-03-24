#!/bin/bash

set -ex

me=$(dirname "$0")
cd "$me"

trap "rm -f ./xfconf/docs.md" EXIT

./gir/generator.py --gir-files-directories ./gir-files.gtk/ ./gir-files.xfce/
./gir/generator.py --strip-docs --embed-docs --gir-files-directories ./gir-files.gtk/ ./gir-files.xfce/
