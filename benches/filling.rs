#![feature(test)]
extern crate trie;
#[macro_use]
extern crate criterion;
extern crate jemallocator;

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

//use trie::experiment1_trie::*;
use trie::DomainLookup;
use trie::gen_seed::*;
use criterion::{Criterion, ParameterizedBenchmark};


fn bench_fill(c: &mut Criterion) {
    let nb_elems_seed = 100i32;

    c.bench(
      "agg:filling tree",
      ParameterizedBenchmark::new("exp 1", |b, n| b.iter(|| {
          let mut root: trie::experiment1_trie::TrieNode<u8> = trie::experiment1_trie::TrieNode::root();
          seed_bench_trie(&mut root, *n);
          seed_known_domain(&mut root);
        }), vec![nb_elems_seed])
      .with_function("exp2", |b, n| b.iter(|| {
        let mut root: trie::experiment2_trie::TrieNode<u8> = trie::experiment2_trie::TrieNode::root();
        seed_bench_trie(&mut root, *n);
        seed_known_domain(&mut root);
      }))
      .with_function("exp3", |b, n| b.iter(|| {
        let mut root: trie::experiment3_trie::TrieNode<u8> = trie::experiment3_trie::TrieNode::root();
        seed_bench_trie(&mut root, *n);
        seed_known_domain(&mut root);
      }))
      .with_function("exp4", |b, n| b.iter(|| {
        let mut root: trie::experiment4_fst::Machine<u8> = trie::experiment4_fst::Machine::new();
        seed_bench_trie(&mut root, *n);
        seed_known_domain(&mut root);
        root.finish();
      }))
      .with_function("exp5", |b, n| b.iter(|| {
        let mut root: trie::experiment5_trie_bitvec::TrieNode<u8> = trie::experiment5_trie_bitvec::TrieNode::root();
        seed_bench_trie(&mut root, *n);
        seed_known_domain(&mut root);
      }))
      .with_function("exp6", |b, n| b.iter(|| {
        let mut root: trie::experiment6_fst_bitvec::Machine<u8> = trie::experiment6_fst_bitvec::Machine::new();
        seed_bench_trie(&mut root, *n);
        seed_known_domain(&mut root);
        root.finish();
      }))
      .with_function("exp7", |b, n| b.iter(|| {
        let mut root: trie::experiment7_regexset::Machine<u8> = trie::experiment7_regexset::Machine::new();
        trie::experiment7_regexset::seed_bench_trie(&mut root, *n);
        trie::experiment7_regexset::seed_known_domain(&mut root);
        root.finish();
      }))
      .with_function("exp8", |b, n| b.iter(|| {
        let mut root: trie::experiment8_trie_cursor::TrieNode<u8> = trie::experiment8_trie_cursor::TrieNode::root();
        seed_bench_trie(&mut root, *n);
        seed_known_domain(&mut root);
      }))
      .with_function("exp9", |b, n| b.iter(|| {
        let mut root: trie::experiment9_hashmap::TrieNode<u8> = trie::experiment9_hashmap::TrieNode::root();
        seed_bench_trie(&mut root, *n);
        seed_known_domain(&mut root);
      }))
      .with_function("exp10", |b, n| b.iter(|| {
        let mut root: trie::experiment10_hashmap_siphash::TrieNode<u8> = trie::experiment10_hashmap_siphash::TrieNode::root();
        seed_bench_trie(&mut root, *n);
        seed_known_domain(&mut root);
      }))
      .with_function("sozu", |b, n| b.iter(|| {
        let mut root: trie::sozu_trie::TrieNode<u8> = trie::sozu_trie::TrieNode::root();
        seed_bench_trie(&mut root, *n);
        seed_known_domain(&mut root);
      }))
      .with_function("linear", |b, n| b.iter(|| {
        let mut root: trie::linear::List<u8> = trie::linear::List::root();
        seed_bench_trie(&mut root, *n);
        seed_known_domain(&mut root);
      }))
      .with_function("hashmap", |b, n| b.iter(|| {
        let mut root: trie::hashmap::Map = trie::hashmap::Map::new();
        seed_bench_trie(&mut root, *n);
        seed_known_domain(&mut root);
      }))
    );
}

criterion_group!(lookup, bench_fill);
criterion_main!(lookup);
