#![feature(test)]
extern crate trie;
#[macro_use]
extern crate criterion;

use trie::experiment2_trie::*;
use criterion::Criterion;

static NB_ELEM_SEED: i32 = 10000;

fn seed_known_domain(root: &mut TrieNode<u8>) {
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

fn bench_look(c: &mut Criterion) {
    c.bench_function("exp 2: registered domains", |b| {
        let mut root: TrieNode<u8> = TrieNode::root();
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
    c.bench_function("exp 2: unregistered domains", |b| {
        let mut root: TrieNode<u8> = TrieNode::root();
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

criterion_group!(lookup, bench_look, bench_lookup_on_unknown);
criterion_main!(lookup);
