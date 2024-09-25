#!/usr/bin/env bash

# set -x

ROOT="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

source "${ROOT}/../common/connector.sh"

clean=
firemultiversx="$ROOT/../firemultiversx"

env_image="multiversx/chain-testnet:latest"

main() {
  pushd "$ROOT" &> /dev/null

  while getopts "hcfous" opt; do
    case $opt in
      h) usage && exit 0;;
      c) clean=true;;
      f) sync_connector && exit 0;;
      o) start_observing_squad && exit 0;;
      u) start_observing_squad && exit 0;;
      s) stop_observing_squad && exit 0;;
      \?) usage_error "Invalid option: -$OPTARG";;
    esac
  done
  shift $((OPTIND-1))
  [[ $1 = "--" ]] && shift

  set -e

  if [[ $clean == "true" ]]; then
    rm -rf firehose-data &> /dev/null || true
  fi

  exec "$firemultiversx" -c "$(basename "$ROOT")".yaml start "$@"
}

start_observing_squad() {
    pushd "$ROOT/../observing-squad"
        env_image=${env_image} ./run.sh setup && env_image=${env_image} ./run.sh run
    popd
}

update_observing_squad() {
    pushd "$ROOT/../observing-squad"
        env_image=${env_image} ./run.sh update
    popd
}

stop_observing_squad() {
    pushd "$ROOT/../observing-squad"
        env_image=${env_image} ./run.sh cleanup
    popd

    if [[ $clean == "true" ]]; then
        rm -rf "$ROOT/OutportBlocks"
        sudo rm -rf "$ROOT/../observing-squad/MyObservingSquad"
    fi
}

usage_error() {
  message="$1"
  exit_code="$2"

  echo "ERROR: $message"
  echo ""
  usage
  exit "${exit_code:-1}"
}

usage() {
  echo "usage: start.sh [-c] [-f] [-o]"
  echo ""
  echo "Start $(basename "$ROOT") environment."
  echo ""
  echo "Options"
  echo "    -c             Clean actual data directory first"
  echo "    -f             Download and setup connector aggregator tool"
  echo "    -o             Setup and start observing squad"
  echo "    -s             Stop observing squad and removing containers"
}

main "$@"
