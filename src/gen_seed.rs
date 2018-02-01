use seed::*;
use uuid::Uuid;
use rand::{XorShiftRng, Rng};

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

