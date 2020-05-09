#!/bin/bash

readonly ROOT=$(cd $(dirname ${BASH_SOURCE:-$0})/..; pwd)

cd $ROOT

hub issue
