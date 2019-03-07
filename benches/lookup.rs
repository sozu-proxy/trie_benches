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
    root.domain_lookup(b"washtucna.obeliskoide.org");
    root.domain_lookup(b"co-adjust.walll-fed.net");
    root.domain_lookup(b"axonne.coadminnistration.gov");
    root.domain_lookup(b"axofugal.obelis.com");
    root.domain_lookup(b"washwomean.coadjuvant.mil");
    root.domain_lookup(b"obeliske.coadjuv.io");
    root.domain_lookup(b"coadunatione.coadministration.th");
    root.domain_lookup(b"axolemma.aaaaxole.ca");
    root.domain_lookup(b"washtail.coadeejute.au");
    root.domain_lookup(b"axolema.washe-pote.rs")
  })
}

fn bench_lookup(c: &mut Criterion) {
    let nb_elems_seed = 1000i32;

    c.bench(
      "agg:registered domains",
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
