#![feature(test)]
extern crate trie;
#[macro_use]
extern crate criterion;
extern crate jemallocator;

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

use trie::linear::*;
use trie::DomainLookup;
use trie::gen_seed::seed_bench_trie;
use criterion::Criterion;

static NB_ELEM_SEED: i32 = 10000;

fn bench_fill(c: &mut Criterion) {
    c.bench_function("linear: filling tree", |b| {

        b.iter(|| {
            let mut root: List<u8> = List::root();
            seed_bench_trie(&mut root, 1000);
        })
    });
}

fn bench_look(c: &mut Criterion) {
    c.bench_function("linear: registered domains", |b| {
        let mut root: List<u8> = List::root();
        seed_known_domain(&mut root);
        seed_bench_trie(&mut root, NB_ELEM_SEED);

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
    c.bench_function("linear: unregistered domains", |b| {
        let mut root: List<u8> = List::root();
        seed_known_domain(&mut root);
        seed_bench_trie(&mut root, NB_ELEM_SEED);

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
