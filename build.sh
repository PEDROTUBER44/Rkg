#!/bin/sh

cargo build --release ;
rm -rf examples/app/* ;
rm -rf examples/source/* ;
cp target/release/rkg examples/app/ ;
cp -r src/ examples/source/ ;
echo "Rkg as successfully compiled" ;