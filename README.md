# Ratel
[![Build Status](https://travis-ci.org/DanielMorton/ratel.svg?branch=master)](https://travis-ci.org/DanielMorton/ratel)
[![CircleCI](https://circleci.com/gh/DanielMorton/ratel.svg?style=svg)](https://circleci.com/gh/DanielMorton/ratel)

Rust Implementation of a Muti-armed Bandit Simulator.

The simulator consists of two main parts,a **Bandit** and an **Agent**.
The agent plays the bandit multiple times in an attempt to learn which bandit
arm provides the highest reward. There are currently three algorithms available
to the agent, and two types of bandit.

## The Bandit

Bandits can have any number of arms, with each arm producing a reward according
to some probability distribution. At the moment, two types of distribution are
available. Either all the arms can have binomial distributions or all arms can
have gaussian distributions. Within those limitations, all choices of
distribution parameter are available.

## The Agent

Agents must determine the bandit arm with the highest reward. The simulator
provides for three strategies. The agent can follow a greedy strategy, always
pulling the arm with the current highest expected reward. The agent can follow
and epsilon-greedy strategy, greedy most of the time but picking a random arm
with some probability. The agent could also choose an optimistic strategy, picking
the arm with the highest value in a range around its expectation.

## Running simulations

For inspiration

## Building Ratel

To build the simulator, clone the repository and run

```cargo build --release```

To run a simulation, use the command

```cargo run --release -- ${YOUR_PARAMETERS}```

