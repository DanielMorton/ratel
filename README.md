# Ratel
[![Build Status](https://travis-ci.org/DanielMorton/ratel.svg?branch=master)](https://travis-ci.org/DanielMorton/ratel)
[![CircleCI](https://circleci.com/gh/DanielMorton/ratel.svg?style=svg)](https://circleci.com/gh/DanielMorton/ratel)
[![](http://meritbadge.herokuapp.com/ratel_bandit)](https://crates.io/crates/ratel_bandit)
[![minimum rustc 1.32](https://img.shields.io/badge/rustc-1.32+-blue.svg)](https://rust-lang.github.io/rfcs/2495-min-rust-version.html)

Rust Implementation of a Muti-armed Bandit Simulator

The simulator consists of two components, a **Bandit** consisting of multiple arms
each of which dispenses rewards according to a probability distribution and an
**Agent** who pulls the arms in an attempt to learn which arm has the highest
average reward.

## The Bandit

A multi-armed bandit consists of a set of arms, each of which, when pulled, gives
a reward according to some probability distribution. Any number of arms is allowed.
There are currently five sets of distributions available; Binomial, Gaussian,
Exponential, Gamma, and LogNormal.. Within those
confines, all choices of distribution parameters are valid.

## The Agent

The agent must determine, by some procedure, which bandit arm produces the highest
average reward. There are currently three strategies implemented. The greedy
algorithm always chooses the arm with the highest estimated average reward. The
epsilon-greedy algorithm follows the greed algorithm most of the time, but
chooses a random arm with some small probability. The optimistic algorithm
chooses the arm whose estimate has the highest upper bound in some confidence
range.

## The Game

The **Game** module manages interactions between the Bandit and the Agent. The
Agent pulls the Bandit's arms a certain number of times. The Game module records
the wins and the rewards for each iteration.

## Building Ratel

To build the simulator simply run

```cargo build --release```

To run the simulator, write a ```main``` module with the desired simulation code.
Then run

```cargo run --releas -- ${PARAMETERS}```

The simulator is designed for maximum flexibility. For inspiration, or to see
how I constructed experiments, see [Ratel-Experiments](https://github.com/DanielMorton/ratel-experiment/tree/master).

## Compatibility

This code is compatible with all versions of Rust from 1.32 to 1.45.