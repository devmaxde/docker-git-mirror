#!/bin/bash

rm -rf $MIRRORS
mkdir -p $MIRRORS

python3 /opt/scripts/init_mirrors.py $MIRRORS $MIRROR_CONFIG
while :
do
    bash /opt/scripts/update_mirrors.sh
    sleep $SLEEP
done
