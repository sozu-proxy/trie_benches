use uuid::Uuid;
use rand::{XorShiftRng, Rng};
use super::DomainLookup;
use std::collections::HashSet;

/// generate a uuid.uuid.tld
pub fn gen_uuid_seed_domain(top_level_domain: &str) -> Vec<u8> {
    let sub_domain_uuid = Uuid::new_v4().simple().to_string();
    let domain_uuid = Uuid::new_v4().simple().to_string();
    format!("{}.{}.{}", sub_domain_uuid, domain_uuid, top_level_domain).into_bytes()
}

/// generate a domain.uuid.tld
pub fn gen_text_seed_domain (tld: &str, domains_list: &Vec<&str>, rand: &mut XorShiftRng) -> Vec<u8> {
    let sub_domain = domains_list[rand.gen_range(0, domains_list.len())];
    let domain = domains_list[rand.gen_range(0, domains_list.len())];
    format!("{}.{}.{}", sub_domain, domain, tld).into_bytes()
}

/// generate a *.uuid.tld domain
pub fn gen_seed_wilcard_domain(top_level_domain: &str) -> Vec<u8> {
    let domain_uuid = Uuid::new_v4().simple().to_string();
    format!("*.{}.{}", domain_uuid, top_level_domain).into_bytes()
}

/// Feed a seed trie with: (nb_elems_seed)
/// 1/3 uui.uuid.tld
/// 1/3 domain_text.uuid.tld
/// 1/3 *.uuid.tld
pub fn seed_bench_trie<T: DomainLookup<u8>>(root: &mut T, nb_elems_seed: i32) {
    let mut random = XorShiftRng::new_unseeded();
    let domains = gen_domains!();
    let tlds = gen_tld!();
    let mut h = HashSet::new();

    for tld in tlds.iter() {
        for _ in 0..nb_elems_seed / 3 {
            root.domain_insert(gen_uuid_seed_domain(tld), 1);
            let text_domain = gen_text_seed_domain(tld, &domains, &mut random);
            if !h.contains(&text_domain) {
              h.insert(text_domain.clone());
              root.domain_insert(text_domain, 2);
            }
            root.domain_insert(gen_seed_wilcard_domain(tld), 2);
        }
    }
}

pub fn seed_known_domain<T: DomainLookup<u8>>(root: &mut T) {
    root.domain_insert(Vec::from(&b"axofugal.obelis.com"[..]), 5);
    root.domain_insert(Vec::from(&b"washtucna.obeliskoide.org"[..]), 5);
    root.domain_insert(Vec::from(&b"co-adjust.walll-fed.net"[..]), 5);
    root.domain_insert(Vec::from(&b"axonne.coadminnistration.gov"[..]), 5);
    root.domain_insert(Vec::from(&b"washwomean.coadjuvant.mil"[..]), 5);
    root.domain_insert(Vec::from(&b"obeliske.coadjuv.io"[..]), 5);
    root.domain_insert(Vec::from(&b"coadunatione.coadministration.th"[..]), 5);
    root.domain_insert(Vec::from(&b"axolemma.aaaaxole.ca"[..]), 5);
    root.domain_insert(Vec::from(&b"washtail.coadeejute.au"[..]), 5);
    root.domain_insert(Vec::from(&b"axolema.washe-pote.rs"[..]), 5);
}
