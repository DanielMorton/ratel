#!/usr/bin/env bash

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
PARENTDIR=$(cd ../ && pwd)

source ${DIR}/docker_env.sh

while :;
do
    case $1 in
        -r|--runs)
            RUNS="$2"
            shift
            ;;
        -n|--num)
            NUM_ITERS="$2"
            shift
            ;;
        -e|--epsilon)
            TYPE="-e $2"
            shift
            ;;
        -g|--greedy)
            TYPE="-g"
            shift
            ;;
        -o|--optimistic)
            TYPE="--pair-optimistic $2"
            shift
            ;;
        *)  break
    esac

    shift
done

docker run ${DOCKER_IMAGE_NAME} -n $NUM_ITERS -r $RUNS $TYPE