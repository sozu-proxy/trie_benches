#![feature(test)]

extern crate trie;
extern crate flame;
extern crate uuid;
extern crate rand;
extern crate test;

use trie::*;
use uuid::Uuid;
use rand::XorShiftRng;
use rand::Rng;
use test::Bencher;

static NB_ELEM_SEED: i32 = 10000;

macro_rules! gen_tld {
    () => (
        vec![
            ".com",
            ".org",
            ".net",
            ".gov",
            ".mil",
            ".io",
            ".th",
            ".ca",
            ".au",
            ".rs",
        ];
    )
}

macro_rules! gen_domains {
    () => (
        vec![
            "axodendrite",
            "axofugal",
            "axogamy",
            "axoid",
            "axoidean",
            "axolemma",
            "axolysis",
            "axolotl",
            "axolotls",
            "axometer",
            "axometry",
            "axometric",
            "axon",
            "coadaptations,",
            "coadapted",
            "coadapting",
            "coadequate",
            "Coady",
            "coadjacence",
            "coadjacency",
            "coadjacent",
            "coadjacently",
            "coadjudicator",
            "coadjument",
            "coadjust",
            "co-adjust",
            "coadjustment",
            "coadjutant",
            "coadjutator",
            "coadjute",
            "coadjutement",
            "coadjutive",
            "coadjutor",
            "coadjutors",
            "coadjutorship",
            "coadjutress",
            "coadjutrice",
            "coadjutrices",
            "coadjutrix",
            "coadjuvancy",
            "coadjuvant",
            "coadjuvate",
            "coadminister",
            "coadministration",
            "coadministrator",
            "coadministratrix",
            "coadmiration",
            "coadmire",
            "coadmired",
            "coadmires",
            "coadmiring",
            "coadmit",
            "coadmits",
            "coadmitted",
            "coadmitting",
            "coadnate",
            "coadore",
            "coadsorbent",
            "coadunate",
            "coadunated",
            "coadunating",
            "coadunation",
            "coadunative",
            "wash-pot",
            "washproof",
            "washrag",
            "washrags",
            "washroad",
            "washroom",
            "washrooms",
            "washshed",
            "washstand",
            "washstands",
            "Washta",
            "washtail",
            "washtray",
            "washtrough",
            "washtub",
            "washtubs",
            "Washtucna",
            "washup",
            "wash-up",
            "washups",
            "washway",
            "washwoman",
            "wall-fed",
            "wall-fight",
            "wallflower",
            "wallflowers",
            "Wallford",
            "wallful",
            "wall-girt",
            "wall-hanging",
            "wallhick",
            "Walli",
            "obelises",
            "obelising",
            "obelisk",
            "obelisked",
            "obelisking",
            "obeliskoid",
            "obelisks",
            "obelism",
            "obelisms",
            "obelize",
            "obelized",
            "obelizes",
            "obelizing",
        ];
    )
}

fn gen_uuid_seed_domain(top_level_domain: &str) -> Vec<u8> {
    let sub_domain_uuid = Uuid::new_v4().simple().to_string();
    let domain_uuid = Uuid::new_v4().simple().to_string();
    format!("{}.{}.{}", sub_domain_uuid, domain_uuid, top_level_domain).into_bytes()
}


fn gen_text_seed_domain (tld: &str, domains_list: &Vec<&str>, rand: &mut XorShiftRng) -> Vec<u8> {
    let sub_domain = domains_list[rand.gen_range(0, domains_list.len())];
    let domain = domains_list[rand.gen_range(0, domains_list.len())];
    format!("{}.{}.{}", sub_domain, domain, tld).into_bytes()
}


fn gen_seed_wilcard_domain(top_level_domain: &str) -> Vec<u8> {
    let domain_uuid = Uuid::new_v4().simple().to_string();
    format!("*.{}.{}", domain_uuid, top_level_domain).into_bytes()
}


fn seed_bench_trie(root: &mut TrieNode<u8>) {
    let mut random = XorShiftRng::new_unseeded();
    let domains = gen_domains!();
    let tlds = gen_tld!();

    for tld in tlds.iter() {
        for _ in 0..NB_ELEM_SEED / 3 {
            root.domain_insert(gen_uuid_seed_domain(tld), 1);
            root.domain_insert(gen_text_seed_domain(tld, &domains, &mut random), 2);
            root.domain_insert(gen_seed_wilcard_domain(tld), 2);
        }
    }
}


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


#[bench]
fn bench_look(b: &mut Bencher) {
    let mut root: TrieNode<u8> = TrieNode::root();
    seed_bench_trie(&mut root);
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
        root.domain_lookup(b"axolema.washe-pote.rs");
    })
}


#[bench]
fn bench_lookup_on_unknown(b: &mut Bencher) {
    let mut root: TrieNode<u8> = TrieNode::root();
    seed_bench_trie(&mut root);
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
        root.domain_lookup(b"book.mac.rs");
    })
}