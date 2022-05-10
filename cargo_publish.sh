#!/bin/sh

set -euxo pipefail

(cd proc-macro
    cargo +stable publish
)

for i in $(seq 10)
do
    cargo +stable publish && break
    sleep 5
done
