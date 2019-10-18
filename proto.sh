#!/bin/bash

set -e

ver="r$(cat VERSION | tr -d '\n')"
proj=tensorflow-serving-client
repo=build/proto
goto=src/main/proto
apis=tensorflow_serving/apis

rm -rf ${repo}
function fetchRepo {
  set -e
  mkdir -p ${repo}/tmp
  git -C ${repo}/tmp \
      clone --depth 1 -b ${ver} \
      https://github.com/tensorflow/${1}.git
  mv ${repo}/tmp/${1}/${2} ${repo}/
  rm -rf ${repo}/tmp
}
fetchRepo tensorflow tensorflow         && echo
fetchRepo serving    tensorflow_serving && echo

rm -rf ${goto}
function importProto {
  if [[ ! -f ${goto}/${1} ]]; then
    echo "${1}"
    mkdir -p ${goto}/${1%/*}
    \cp ${repo}/${1} ${goto}/${1}
    local proto=( $(grep '^import ' ${goto}/${1} \
                  | grep -Eo '((\w|\d|_|-)+(/|\.))+proto') )
    for p in ${proto[@]}; do
      if [[ "${p:0:10}" == "tensorflow" ]]; then
        importProto ${p}
      fi
    done
  fi
}
for i in ${repo}/${apis}/*.proto; do
  importProto ${i#${repo}/}
done
