# Domain of minchain

## Block structure

Each block contains a string field which can hold any arbitrary data. Besides the content each block has a header with the purpose of validating (singing) the block. The header contains:

- the hash of the previous block
- an arbitrary number (salt)
- and the hash of the current block

The hash is calculated from the whole block content, using SHA256 algorithm.

## Genesis block

The genesis block is the first block in the chain.

It has no real hashes, salt or content. But also, it's never validated. By design, no real data can be added before a genesis block.

## Chain

The chain is just a list of blocks. Its functions are limited to:

- adding a genesis block
- storing data
- overriding the whole chain with a new state received from a more up-to-date node

## PTP network

The real value of blockchain that it is a distributed across independent machines, which together form a peer-to-peer network.

This project implements a really stupid version of such a network.

After starting a node, you can manually register peers. Right now these peers won't automatically talk with each other. They could exchange their peer list, or notify each others about new blocks, but right now none of this happens. 

The only way to synchronize states between nodes is to manually execute the `sync` command on a node: as a result, it will get the state from all of its peers, and if it finds a more up-to-date version, it will drop its own in favor of that.

## Branches and syncronization

As there are multiple nodes in the network running simultaneously, it is possible that different branches of the same chain emerge.

An easy way to reproduce this:

```bash
A> cargo run 8081
B> cargo run 8082
B> addpeer localhost:8081

A> addgen
B> addgen
A> addcontent a
B> addcontent b
A> addcontent c

A> print
B> print
```

Now the print from `A` instance will print three blocks, and the print from `B` instance will print two blocks.

This is an anomaly, but such thing happens eg. on the bitcoin network as well. A very basic resolution strategy is just pick the longer chain in this case, and drop the shorter.

This means that since we added `A` as a peer for `B`, if we execute the `sync` command on `B`, its state (`[b]`) will be overwritten by the longer state (`[a, c]`).

But wait, what happens with `b` in this case? Well, it just disappears, it has to be mined again. Unfortunately, this is still a thing which happens on the bitcoin network as well. Blockchain can provide only probabilistic guarantees that a stored value will remain in the chain later as well. I read somewhere that a common heuristic in case of the bitcoin network is wait until 3-4 nodes acknowledged a transaction, and then it can be considered as done.

## Proof of Work

Minchain uses a basic one-liner Proof of Work algorithm.

Only those blocks are considered valid where the hash of the block ends with `00`.

In case of minchain, this "algorithm" does not really have any added value or role, but generally in the world of blockchains it would serve two purposes (as I understand):

- as it increases the required time for mining a new block, it decreases the level of parallelism, which will lead to less different branches.
- as we always choose the longest chain for conflict resolution, it also means that we always choose the one which required the most computing power (= energy = money). So if someone wants to mess up our database, the bigger the change, the more money needed for it. In a real network, the PoW algorithm can be finetuned in a way (eg. by setting how many trailing zeros the hash should have) that messing with the data does not worth it.

Worth mentioning that there are better (more complex, but less resource-consuming) proof of * algorithms, eg. proof of stake which is planned to use in the ethereum chain in the future.