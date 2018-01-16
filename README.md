# Trie benches

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