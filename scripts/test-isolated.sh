#!/bin/bash
set -euo pipefail
# Must be launched by root as: unshare --net bash test-isolated.sh <test-binary> [filter]
# This namespace has no connection to the user's LAN.
ip link set lo up
ip link add mdnstest type dummy
ip addr add 192.0.2.1/24 dev mdnstest
ip link set mdnstest up multicast on
ip route add 224.0.0.0/4 dev mdnstest
export BIFROST_ISOLATED_TEST=1
filter="${2:-patch_regression}"
if [ "$filter" = all ]; then filter=""; fi
"$1" "$filter" --test-threads=1 --nocapture
