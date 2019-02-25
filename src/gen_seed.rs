use uuid::Uuid;
use rand::{XorShiftRng, Rng};
use super::DomainLookup;

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

    for tld in tlds.iter() {
        for _ in 0..nb_elems_seed / 3 {
            root.domain_insert(gen_uuid_seed_domain(tld), 1);
            root.domain_insert(gen_text_seed_domain(tld, &domains, &mut random), 2);
            root.domain_insert(gen_seed_wilcard_domain(tld), 2);
        }
    }
}
