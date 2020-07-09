#!/usr/bin/env bash

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
            TYPE="--pair-epsilon $2"
            shift
            ;;
        -g|--greedy)
            TYPE="--pair-greedy"
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

cargo run --release -- -r $RUNS -n $NUM_ITERS $TYPE