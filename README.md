# DodoPoW - Cuckoo Cycle-like PoW mechanism for CPU execution

This library provides a PoW task similar to the
[Cockoo Cycle](https://github.com/tromp/cuckoo)'s lean mining, prioritizing
single-core CPU execution. The purpose is to fight against GPUs and ASICs and
improve networks decentralization by utilizing general purpose CPUs which all
the people have.

# Core principle

DodoPoW builds a [bipartite graph](https://en.wikipedia.org/wiki/Bipartite_graph)
of `2N` nodes and `N = 2^n` edges, where `n` is configurable and impacts the
amount of used RAM and compute time. The PoW task is to find a cycle of length
`diff` in such a graph which is sampled from any seeded RNG. `diff` scaling
reduces probability of finding such cycles.

In blockchain applications this algorithm can be used with standardized `n`
value. A miner can struggle to find a cycle with pre-defined `diff` value, thus
it's recommended to scale miner's payment exponentially-like with the `diff`
value growth and allow them to choose this value as they like.

> Note: the implementation in this library is **not** optimized for performance,
> its goal is to be correct. Extra difficulty tricks can be used to adjust
> the computation speed limit in runtime.

Author: [Nikita Podvirnyi](https://github.com/krypt0nn)\
Licensed under [MIT](LICENSE)
