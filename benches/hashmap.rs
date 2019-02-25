#![feature(test)]
#[macro_use]
extern crate trie;
#[macro_use]
extern crate criterion;
extern crate jemallocator;
extern crate rand;

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

use criterion::Criterion;
use trie::gen_seed::*;
use std::collections::HashMap;
use rand::XorShiftRng;

static NB_ELEM_SEED: i32 = 10000;

pub struct Map(HashMap<Vec<u8>, u8>);

impl Map {
  pub fn domain_insert(&mut self, key: Vec<u8>, value: u8) {
    let mut partial_key = key.clone();
    partial_key.reverse();
    self.0.insert(partial_key, value);
  }

  pub fn domain_lookup(&self, key: &[u8]) -> Option<(Vec<u8>, u8)> {
    let mut partial_key = key.to_vec();
    partial_key.reverse();
    self.0.get(&partial_key).map(|v| (key.to_vec(), *v))
  }
}

fn seed_known_domain(root: &mut Map) {
    root.domain_insert(Vec::from(&b"axofugal.obelis.com"[..]), 5);
    root.domain_insert(Vec::from(&b"Washtucna.obeliskoide.org"[..]), 5);
    root.domain_insert(Vec::from(&b"co-adjust.walll-fed.net"[..]), 5);
    root.domain_insert(Vec::from(&b"axonne.coadminnistration.gov"[..]), 5);
    root.domain_insert(Vec::from(&b"washwomean.coadjuvant.mil"[..]), 5);
    root.domain_insert(Vec::from(&b"obeliske.coadjuv.io"[..]), 5);
    root.domain_insert(Vec::from(&b"coadunatione.coadministration.th"[..]), 5);
    root.domain_insert(Vec::from(&b"axolemma.aaaaxole.ca"[..]), 5);
    root.domain_insert(Vec::from(&b"washtail.coadeejute.au"[..]), 5);
    root.domain_insert(Vec::from(&b"axolema.washe-pote.rs"[..]), 5);
}

pub fn seed_bench_trie(root: &mut Map, nb_elems_seed: i32) {
    let mut random = XorShiftRng::new_unseeded();
    let domains = gen_domains!();
    let tlds = gen_tld!();

    for tld in tlds.iter() {
        for _ in 0..nb_elems_seed / 3 {
            root.domain_insert(gen_uuid_seed_domain(tld), 1);
            root.domain_insert(gen_text_seed_domain(tld, &domains, &mut random), 2);
            root.domain_insert(gen_seed_wilcard_domain(tld), 2);
        }
    }
}

fn bench_fill(c: &mut Criterion) {
    c.bench_function("hashmap: filling tree", |b| {

        b.iter(|| {
            let mut root = Map(HashMap::new());
            seed_bench_trie(&mut root, 1000);
        })
    });
}

fn bench_look(c: &mut Criterion) {
    c.bench_function("hashmap: registered domains", |b| {
        let mut root = Map(HashMap::new());
        seed_bench_trie(&mut root, NB_ELEM_SEED);
        seed_known_domain(&mut root);

        b.iter(|| {
            root.domain_lookup(b"Washtucna.obeliskoide.org");
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
    });
}

fn bench_lookup_on_unknown(c: &mut Criterion) {
    c.bench_function("hashmap: unregistered domains", |b| {
        let mut root = Map(HashMap::new());
        seed_bench_trie(&mut root, NB_ELEM_SEED);
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
    });
}

criterion_group!(lookup, bench_fill, bench_look, bench_lookup_on_unknown);
criterion_main!(lookup);
