# Sozu routing benchmarks

This project explores optimizations for the SNI certificate lookup and HTTP router
algorithm in the [sozu HTTP reverse proxy](https://github.com/sozu-proxy/sozu).

In sozu, configuration changes happen often at runtime, and it is designed as a
load balancer for tens of thousands of different applications and routes.

We need a compact way to store those routes, that can be updated quickly, and
for which lookup is fast. As a point of comparison, a fast HTTP parser takes
150-300ns to parse all the headers.

The current implementation in sozu is based on a trie that works on reversed
domain names, ie for the "example.com" domain, the key will be "moc.elpmaxe".
It matches on prefixes, handles the special case of wildcard domain names
for SNI. Once the trie has found a matching node for the domain, the node
holds a list of URL prefixes that match to applications.

This has served sozu well in the context of Clever Cloud, where we have a lot
of different domains, but the more common use case is to have a few domains,
but a lot of URL/app matches.

We also need more flexible matching patterns, like regular expressions on domain
names and URL.

## Routing experiments

this repository holds various experiments around routing and trie optimizations,
mostly working on domain names.

Some base examples are available:
- the "sozu" version uses the trie available in sozu 0.11
- the "hashmap" version uses a hashmap to lookup the route (so no prefix or wildcard usage)
- the "linear" version tests routes one after the other until one matches

### Exp 1: trie with vector of `(key, child)`

This is a naive implementation of a trie, with wildcard domain support. It is easy
to write, and easy to update in place.

### Exp 2: trie with vector of key and vector of children

Builds on exp1 but separates the keys

This is the current version on sozu 0.11

### Exp 3: keep a local prefix, and only the first byte of each child key

this implementation leverages common prefixes to only match them once.
And the children will necessarily have differences on the first byte of the
rest of their key.

### Exp 4: state machines

Experiment 4 uses the [fst](https://crates.io/crates/fst) crate. This comes with a
tradeoff: since the list of keys has to be sorted, any edition requires rebuilding
the entire machine. We could live with this tradeoff, by making the master process
write a new state machine while the workers use the previous one.

### Exp 5: trie with bitvec

this one uses the [bitvec](https://crates.io/crates/bitvec) crate to test a language
restriction: domain name uses alphanumerical characters (we can limit to lowercase)
and '-', '.', '*', and '_' (the underscore is not in the spec, but we've seen usage
in the wild.

We can convert those caracters to a 6 bit encoding, and have a more compact tree,
with potentially faster lookup.


### Exp 6: state machine with bitvec

Experiment 6 reuses the fst from exp 4 with the bitvec language from exp 5.

### Exp 7: regex set

this version makes a regex out of each route, and uses a `RegexSet` from the
[regex](https://crates.io/crates/regex) crate to assemble them.

### Exp 8: specialised trie

This is a more complex version of the exp 3 trie. It uses a cursor structure made from
the domain name and the URLL, and can match prefixes, regexes and SNI wildcard on it.
As a result, the trie nodes are a bit larger

### Exp 9: tree of hashmaps

This version cuts domain names in labels, and each node contains a hashmap of the next
labels to the corresponding children. It uses the [hashbrown](https://crates.io/crates/hashbrown)
crate for its fast hashmap implementation (swisstable)

### Exp 10: tree of hashmaps with SipHash

There's a case where exp 9 could be vulnerable to hash flooding, since at Clever Cloud, our users
control which domain names are used. So this one takes the code of exp 9 with the siphash
hash algorithm (currently used as default hasher in Rust)

## Benchmark results

tested on a MacBook Pro (Retina, 15-inch, Late 2013), CPU 2,3 GHz Intel Core i7

We have 3 main benchmarks to compare results:
- time to fill the structure with 100 entries, by incremental edition
- time to lookup a known key in a structure of 1000 entries
- time to lookup an unknown key in a structue of 1000 entries
-
|                           | fill structure | lookup known key  | lookup unknown key |
| ------------------------- | -------------- | ----------------- | ------------------ |
| linear                    |     1.35 ms    |       31 us       |     34.31 us       |
| hashmap                   |     1.42 ms    |        7 ns       |         1 ns       |
| current sozu impl         |     1.60 ms    |      182 ns       |       229 ns       |
| 1: trie                   |     1.82 ms    |      138 ns       |        80 ns       |
| 2: trie+separated keys    |     1.95 ms    |      144 ns       |        80 ns       |
| 3: optimized trie         |     1.72 ms    |       74 ns       |        52 ns       |
| 4: fst                    |     7.58 ms    |      354 ns       |       117 ns       |
| 5: trie+bitvec            |    12.89 ms    |     5.79 us       |      2.08 us       |
| 6: fst+bitvec             |    14.69 ms    |     5.82 us       |      2.12 us       |
| 7: regexset               |    33.10 ms    |     1.15 ms       |      1.19 ms       |
| 8 trie cursor             |     1.89 ms    |      119 ns       |        57 ns       |
| 9: hashmap tree           |     1.73 ms    |       65 ns       |        28 ns       |
| 10 hashmap tree + siphash |     1.89 ms    |      109 ns       |        57 ns       |

We also have a version with larger values, but eliminating the slowest versions (experiments
4, 5, 6 and 7), as they would make a the benchmark take too long.
- time to fill the structure with 1000 entries, by incremental edition
- time to lookup a known key in a structure of 10000 entries
- time to lookup an unknown key in a structue of 10000 entries

|                           | fill structure | lookup known key  | lookup unknown key |
| ------------------------- | -------------- | ----------------- | ------------------ |
| linear                    |    12.68 ms    |   560.68 us       |       586 us       |
| hashmap                   |    13.88 ms    |        7 ns       |         1 ns       |
| current sozu impl         |    15.71 ms    |      230 ns       |       223 ns       |
| 1: trie                   |    18.14 ms    |      158 ns       |        81 ns       |
| 2: trie+separated keys    |    19.17 ms    |      159 ns       |        87 ns       |
| 3: optimized trie         |    16.45 ms    |       83 ns       |        51 ns       |
| 8 trie cursor             |    19.62 ms    |      127 ns       |        58 ns       |
| 9: hashmap tree           |    16.50 ms    |       64 ns       |        28 ns       |
| 10 hashmap tree + siphash |    18.34 ms    |      113 ns       |        58 ns       |

We can see that the trie implementations stay stable when we increase the number of
entries.
They do not perform exactly the same tasks (experiment 8 especially is testing regexps
along with the prefixes).
While version 9 and 10 looks like a quick and dirty fix, it's actually easy to write and
maintain, its performance is very stable, and we can make Siphash usage configurable.
So we will recommend adapting the tree of hashmaps as new solution.
