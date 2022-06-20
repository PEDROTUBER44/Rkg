#!/bin/sh

dd if=/dev/zero of=rkg.rkg bs=1k count=16384 ;
mkdosfs rkg.rkg ;
sudo mkdir -p /tmp/rkg/ ;
sudo mount -o loop rkg.rkg /tmp/rkg/ ;
cp -r examples/ /tmp/rkg/ ;
umount /tmp/rkg/ ;
echo "Rocket package created successfully"