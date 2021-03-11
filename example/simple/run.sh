#!/bin/bash

set -e

SCRIPT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/" && pwd)"


function logger(){
    for i in {1..100}; do
        echo "$1 run #$i";
        sleep 0.2
    done
}

rm -f $SCRIPT_ROOT/*.log

$SCRIPT_ROOT/../../target/debug/rlog &
RLOG="$!"

logger "frontend" > $SCRIPT_ROOT/frontend.log &
logger "backend" > $SCRIPT_ROOT/backend.log &
logger "database" > $SCRIPT_ROOT/database.log &

wait

kill -9 $RLOG

echo "Done"
