# Basic concepts

(*My understanding after few days of searching around*)

## Blockchain

A blockchain is a distributed database, in which the data is stored on a network of independent nodes, and a whole replica of the data is stored on all the nodes.

The data is immutable, meaning that once something is saved on the blockchain, it cannot be erased again.

## Trustless consensus

In this network there are no leader and worker nodes, no leader election etc. Instead of this, all the decisions are made "together" by the nodes. The algorithm of the nodes ensures that there will be always a consensus between them.

This algorithm has to be "trustless" - meaning, that it has to be resilient against such attacks where some subset of the nodes run an altered code which aims to insert or alter data in an invalid way.

In theory until the number of the malicious nodes is lower then 50%+1, the network will be able to enforce the rules written in the original code.

## Cryptocurrency

Cryptocurrency is a specific type of applications, built on top of the blockchain technology. There are other applications, like NFTs, smart contracts etc.

In this project I did not build any application layer, this is just a dummy blockchain which can store and serve data.