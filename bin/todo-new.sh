#!/bin/bash

readonly ROOT=$(cd $(dirname ${BASH_SOURCE:-$0})/..; pwd)

cd $ROOT

hub issue create -a $(git config user.name) -l 'todo' -m "$1"
