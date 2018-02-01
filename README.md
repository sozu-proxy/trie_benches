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
