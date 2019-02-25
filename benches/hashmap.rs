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
use trie::{DomainLookup, InsertResult, RemoveResult, Key};
use trie::hashmap::*;

use rand::XorShiftRng;

static NB_ELEM_SEED: i32 = 10000;

fn bench_fill(c: &mut Criterion) {
    c.bench_function("hashmap: filling tree", |b| {

        b.iter(|| {
            let mut root = Map::new();
            seed_bench_trie(&mut root, 1000);
        })
    });
}

fn bench_look(c: &mut Criterion) {
    c.bench_function("hashmap: registered domains", |b| {
        let mut root = Map::new();
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
        let mut root = Map::new();
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
