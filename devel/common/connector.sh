#!/usr/bin/env bash

sync_connector() {
  # TODO: change to use latest tag
  local branch=0.0.6

  local dir_name=connector-repo

  git clone \
    https://github.com/multiversx/mx-chain-ws-connector-firehose-go ${dir_name} \
      --branch=${branch} \
      --single-branch \
      --depth=1

  pushd "${dir_name}/cmd/connector" &> /dev/null
  go build
  popd

  cp ${dir_name}/cmd/connector/connector ${ROOT} 
  cp -r ${dir_name}/cmd/connector/config ${ROOT}

  rm -rf ${dir_name}
}

