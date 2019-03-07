#![feature(test)]
extern crate trie;
#[macro_use]
extern crate criterion;
extern crate jemallocator;

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

//use trie::experiment1_trie::*;
use trie::DomainLookup;
use trie::gen_seed::seed_bench_trie;
use criterion::{Criterion, ParameterizedBenchmark};


fn bench_fill(c: &mut Criterion) {
    let nb_elems_seed = 1000i32;

    c.bench(
      "agg:filling tree",
      ParameterizedBenchmark::new("exp 1", |b, n| b.iter(|| {
          let mut root: trie::experiment1_trie::TrieNode<u8> = trie::experiment1_trie::TrieNode::root();
          seed_bench_trie(&mut root, *n);
        }), vec![nb_elems_seed])
      .with_function("exp2", |b, n| b.iter(|| {
        let mut root: trie::experiment2_trie::TrieNode<u8> = trie::experiment2_trie::TrieNode::root();
        seed_bench_trie(&mut root, *n);
      }))
      .with_function("exp3", |b, n| b.iter(|| {
        let mut root: trie::experiment3_trie::TrieNode<u8> = trie::experiment3_trie::TrieNode::root();
        seed_bench_trie(&mut root, *n);
      }))
      .with_function("exp4", |b, n| b.iter(|| {
        let mut root: trie::experiment4_fst::Machine<u8> = trie::experiment4_fst::Machine::new();
        seed_bench_trie(&mut root, *n);
        root.finish();
      }))
      .with_function("sozu", |b, n| b.iter(|| {
        let mut root: trie::sozu_trie::TrieNode<u8> = trie::sozu_trie::TrieNode::root();
        seed_bench_trie(&mut root, *n);
      }))
      .with_function("linear", |b, n| b.iter(|| {
        let mut root: trie::linear::List<u8> = trie::linear::List::root();
        seed_bench_trie(&mut root, *n);
      }))
      .with_function("hashmap", |b, n| b.iter(|| {
        let mut root: trie::hashmap::Map = trie::hashmap::Map::new();
        seed_bench_trie(&mut root, *n);
      }))
    );
}

criterion_group!(lookup, bench_fill);
criterion_main!(lookup);
