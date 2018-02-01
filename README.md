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
exp 1: registered domai time:   [892.14 ns 918.02 ns 946.99 ns]
                        change: [-4.8929% -1.7991% +1.2176%] (p = 0.26 > 0.05)
                        No change in performance detected.
Found 6 outliers among 100 measurements (6.00%)
  6 (6.00%) high mild

exp 1: unregistered dom time:   [702.82 ns 724.92 ns 748.72 ns]
                        change: [-4.3137% -0.9070% +2.5734%] (p = 0.62 > 0.05)
                        No change in performance detected.
Found 8 outliers among 100 measurements (8.00%)
  8 (8.00%) high mild

     Running target/release/deps/experiment2-80fc56ecd4361311
exp 2: registered domai time:   [829.67 ns 856.88 ns 888.09 ns]
                        change: [-6.0153% -2.4433% +1.0669%] (p = 0.18 > 0.05)
                        No change in performance detected.
Found 8 outliers among 100 measurements (8.00%)
  5 (5.00%) high mild
  3 (3.00%) high severe

exp 2: unregistered dom time:   [624.92 ns 646.62 ns 672.80 ns]
                        change: [-12.627% -6.0033% -0.1861%] (p = 0.07 > 0.05)
                        No change in performance detected.
Found 8 outliers among 100 measurements (8.00%)
  6 (6.00%) high mild
  2 (2.00%) high severe

     Running target/release/deps/sozu-f6321307a9273616
sozu: registered domain time:   [1.0801 us 1.1254 us 1.1791 us]
                        change: [-11.330% -5.6272% +0.1208%] (p = 0.07 > 0.05)
                        No change in performance detected.
Found 9 outliers among 100 measurements (9.00%)
  6 (6.00%) high mild
  3 (3.00%) high severe

sozu: unregistered doma time:   [1.1163 us 1.1497 us 1.1902 us]
                        change: [-10.623% -3.8521% +4.5532%] (p = 0.33 > 0.05)
                        No change in performance detected.
Found 11 outliers among 100 measurements (11.00%)
  3 (3.00%) high mild
  8 (8.00%) high severe
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
