#!/bin/bash

rm -rf $MIRRORS
mkdir -p $MIRRORS

cd $MIRRORS
python /opt/scripts/init_mirrors.py

while :
do
    bash /opt/scripts/update_mirrors.sh
    sleep $SLEEP
done