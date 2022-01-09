#!/bin/sh

cargo build --release ;
rm -rf app/* ;
cp target/release/rkg app/ ;
rm -rf target/ ;
echo "rkg successfully compiled" ;