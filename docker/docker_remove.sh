#!/usr/bin/env bash

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"

source ${DIR}/docker_env.sh

docker rmi -f ${DOCKER_IMAGE_NAME}