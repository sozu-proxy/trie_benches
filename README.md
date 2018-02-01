# Trie benches

This project explores optimizations fo sozu's certificate lookup algorithm.
It is based on a trie that looks up a reversed domain, ie for the "example.com"
domain, the key will be "moc.elpmaxe".

The current implementation is a kind of radix trie with a special case for wildcard
domains. Some nodes could have "\*.example.com" as key, and the lookup operation
must match them to "test.example.com".

In general, we do not care much about the cost of adding or removing an element.
Those operations are much less frequent (happen only during configuration change events)
than lookups (happen at new connections).

The structure does not need to be extremely space efficient (certificates and connection
buffers are much more important in sozu), but lookup needs to be as fast as possible.

## Current benchmarks

Measured on a MacBook Pro (Retina, 15-inch, Late 2013), CPU 2,3 GHz Intel Core i7

```
exp 1: registered domai time:   [803.38 ns 832.43 ns 868.33 ns]
Found 11 outliers among 100 measurements (11.00%)
  7 (7.00%) high mild
  4 (4.00%) high severe

exp 1: unregistered dom time:   [644.84 ns 682.17 ns 724.63 ns]
Found 8 outliers among 100 measurements (8.00%)
  6 (6.00%) high mild
  2 (2.00%) high severe

     Running target/release/deps/experiment2-80fc56ecd4361311
exp 2: registered domai time:   [853.15 ns 879.78 ns 911.60 ns]
Found 14 outliers among 100 measurements (14.00%)
  5 (5.00%) high mild
  9 (9.00%) high severe

exp 2: unregistered dom time:   [672.55 ns 698.91 ns 730.22 ns]
Found 6 outliers among 100 measurements (6.00%)
  2 (2.00%) high mild
  4 (4.00%) high severe

     Running target/release/deps/sozu-f6321307a9273616
sozu: registered domain time:   [1.0456 us 1.0763 us 1.1160 us]
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) high mild
  5 (5.00%) high severe

sozu: unregistered doma time:   [1.0591 us 1.0868 us 1.1197 us]
Found 11 outliers among 100 measurements (11.00%)
  5 (5.00%) high mild
  6 (6.00%) high severe
```

## Measuring things with perf

``` bash
perf stat -B -e cache-misses,\
cycles,\
instructions,\
branches,\
faults \
./target/release/examples/lookup
```

Result with a `Intel(R) Core(TM) i7-4750HQ CPU @ 2.00GHz`

``` bash
 Performance counter stats for './target/release/examples/lookup':

           259,269      cache-misses:u                                              
       529,144,357      cycles:u                                                    
     1,249,387,240      instructions:u            #    2.36  insn per cycle         
       226,593,988      branches:u                                                  
                95      faults:u                                                    

       0.175661766 seconds time elapsed
```
