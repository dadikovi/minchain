# Limitations

## Functional limitations

<dl>
<dt>No persistence</dt>
<dd>If all nodes in the network are shut down, data is lost</dd>
<dt>No automatic syncing between nodes</dt>
<dd>usually after a new block is mined, the miner node notifies everyone else in the network.</dd>
<dt>No exchanging of peers</dt> 
<dd>you have to register all peers manually</dd>
</dl>

## Scalebility

*By design, this project could not scale to a level where it can actually operate as a blockchain network.*

<dl>
<dt>The PoW algorithm is really "week"</dt>
<dd>mining a new block takes only a few ms, so in a real-life use case the number of parallel branches would be to high.</dd>
<dd>But then if the algorithm would be parametrized to be more complex, saving new data would take lot more time, making the API really not convenient. This is because receiving new data and mining are tight together. In bitcoin this probem is solved by the mempool.</dd>
</dl>