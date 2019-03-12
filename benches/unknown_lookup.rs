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
use criterion::{Criterion, Bencher, ParameterizedBenchmark};

fn lookup<T: DomainLookup<u8>>(mut root: T, b: &mut Bencher, nb_elem_seed: i32) {
  seed_bench_trie(&mut root, nb_elem_seed);
  seed_known_domain(&mut root);

  b.iter(|| {
    root.domain_lookup(b"sozu.org");
    root.domain_lookup(b"yolo.toto.net");
    root.domain_lookup(b"foo.kill.gov");
    root.domain_lookup(b"riche.riche.com");
    root.domain_lookup(b"unknown.zelda.mil");
    root.domain_lookup(b"tracktl.yolo.io");
    root.domain_lookup(b"rebase.arnaud.th");
    root.domain_lookup(b"never.gonna.ca");
    root.domain_lookup(b"clever.cloud.au");
    root.domain_lookup(b"book.mac.rs")
  })
}

fn bench_lookup(c: &mut Criterion) {
    let nb_elems_seed = 1000i32;

    c.bench(
      "agg:unregistered domains",
      ParameterizedBenchmark::new("exp 1", |mut b, n| {
        let root: trie::experiment1_trie::TrieNode<u8> = trie::experiment1_trie::TrieNode::root();
        lookup(root, &mut b, *n);
      }, vec![nb_elems_seed])
      .with_function("exp2", |mut b, n| {
        let root: trie::experiment2_trie::TrieNode<u8> = trie::experiment2_trie::TrieNode::root();
        lookup(root, &mut b, *n);
      })
      .with_function("exp3", |mut b, n| {
        let root: trie::experiment3_trie::TrieNode<u8> = trie::experiment3_trie::TrieNode::root();
        lookup(root, &mut b, *n);
      })
      .with_function("exp4", |b, n| {
        let mut root: trie::experiment4_fst::Machine<u8> = trie::experiment4_fst::Machine::new();
        seed_bench_trie(&mut root, *n);
        seed_known_domain(&mut root);
        root.finish();

        b.iter(|| {
          root.domain_lookup(b"sozu.org");
          root.domain_lookup(b"yolo.toto.net");
          root.domain_lookup(b"foo.kill.gov");
          root.domain_lookup(b"riche.riche.com");
          root.domain_lookup(b"unknown.zelda.mil");
          root.domain_lookup(b"tracktl.yolo.io");
          root.domain_lookup(b"rebase.arnaud.th");
          root.domain_lookup(b"never.gonna.ca");
          root.domain_lookup(b"clever.cloud.au");
          root.domain_lookup(b"book.mac.rs")
        })
      })
      /*.with_function("exp5", |mut b, n| {
        let root: trie::experiment5_trie_bitvec::TrieNode<u8> = trie::experiment5_trie_bitvec::TrieNode::root();
        lookup(root, &mut b, *n);
      })
      .with_function("exp6", |mut b, n| {
        let mut root: trie::experiment6_fst_bitvec::Machine<u8> = trie::experiment6_fst_bitvec::Machine::new();
        seed_bench_trie(&mut root, *n);
        seed_known_domain(&mut root);
        root.finish();

        b.iter(|| {
          root.domain_lookup(b"sozu.org");
          root.domain_lookup(b"yolo.toto.net");
          root.domain_lookup(b"foo.kill.gov");
          root.domain_lookup(b"riche.riche.com");
          root.domain_lookup(b"unknown.zelda.mil");
          root.domain_lookup(b"tracktl.yolo.io");
          root.domain_lookup(b"rebase.arnaud.th");
          root.domain_lookup(b"never.gonna.ca");
          root.domain_lookup(b"clever.cloud.au");
          root.domain_lookup(b"book.mac.rs")
        })
      })
      */
      .with_function("exp7", |b, n| {
        let mut root: trie::experiment7_regexset::Machine<u8> = trie::experiment7_regexset::Machine::new();
        trie::experiment7_regexset::seed_bench_trie(&mut root, *n);
        trie::experiment7_regexset::seed_known_domain(&mut root);
        root.finish();

        b.iter(|| {
          root.domain_lookup(b"sozu.org");
          root.domain_lookup(b"yolo.toto.net");
          root.domain_lookup(b"foo.kill.gov");
          root.domain_lookup(b"riche.riche.com");
          root.domain_lookup(b"unknown.zelda.mil");
          root.domain_lookup(b"tracktl.yolo.io");
          root.domain_lookup(b"rebase.arnaud.th");
          root.domain_lookup(b"never.gonna.ca");
          root.domain_lookup(b"clever.cloud.au");
          root.domain_lookup(b"book.mac.rs")
        });
      })
      .with_function("sozu", |mut b, n| {
        let root: trie::sozu_trie::TrieNode<u8> = trie::sozu_trie::TrieNode::root();
        lookup(root, &mut b, *n);
      })
      .with_function("linear", |mut b, n| {
        let root: trie::linear::List<u8> = trie::linear::List::root();
        lookup(root, &mut b, *n);
      })
      .with_function("hashmap", |mut b, n| {
        let root: trie::hashmap::Map = trie::hashmap::Map::new();
        lookup(root, &mut b, *n);
      })
    );
}

criterion_group!(benches, bench_lookup);
criterion_main!(benches);
