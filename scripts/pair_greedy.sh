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
        *)  break
    esac

    shift
done

cargo run --release -- -r $RUNS -n $NUM_ITERS --pair-greedy