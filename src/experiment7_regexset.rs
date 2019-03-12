//! try with the fst crate
//!
//! this example uses a state machine that must be completely regenerated
//! on each change, and keys must be inserted in order

use std::fmt::{Debug,Display};
use regex::bytes::{RegexSet,RegexSetBuilder};
use uuid::Uuid;
use rand::{XorShiftRng, Rng};
use std::collections::HashSet;

use super::{Key, KeyValue, InsertResult, RemoveResult, DomainLookup};

pub fn gen_uuid_seed_domain(top_level_domain: &str) -> Vec<u8> {
    let sub_domain_uuid = Uuid::new_v4().simple().to_string();
    let domain_uuid = Uuid::new_v4().simple().to_string();
    format!("${}.{}.{}^", sub_domain_uuid, domain_uuid, top_level_domain).into_bytes()
}

pub fn gen_text_seed_domain (tld: &str, domains_list: &Vec<&str>, rand: &mut XorShiftRng) -> Vec<u8> {
    let sub_domain = domains_list[rand.gen_range(0, domains_list.len())];
    let domain = domains_list[rand.gen_range(0, domains_list.len())];
    format!("${}.{}.{}^", sub_domain, domain, tld).into_bytes()
}

/// generate a *.uuid.tld domain
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
              //println!("inserting {}", std::str::from_utf8(&text_domain).unwrap());
              root.domain_insert(text_domain, 2);
            }
            //root.domain_insert(gen_seed_wilcard_domain(tld), 2);
        }
    }
}

pub fn seed_known_domain<T: DomainLookup<u8>>(root: &mut T) {
    root.domain_insert(Vec::from(&b"$axofugal.obelis.com^"[..]), 5);
    root.domain_insert(Vec::from(&b"$Washtucna.obeliskoide.org^"[..]), 5);
    root.domain_insert(Vec::from(&b"$co-adjust.walll-fed.net^"[..]), 5);
    root.domain_insert(Vec::from(&b"$axonne.coadminnistration.gov^"[..]), 5);
    root.domain_insert(Vec::from(&b"$washwomean.coadjuvant.mil^"[..]), 5);
    root.domain_insert(Vec::from(&b"$obeliske.coadjuv.io^"[..]), 5);
    root.domain_insert(Vec::from(&b"$coadunatione.coadministration.th^"[..]), 5);
    root.domain_insert(Vec::from(&b"$axolemma.aaaaxole.ca^"[..]), 5);
    root.domain_insert(Vec::from(&b"$washtail.coadeejute.au^"[..]), 5);
    root.domain_insert(Vec::from(&b"$axolema.washe-pote.rs^"[..]), 5);
}

pub struct Machine<V> {
  index: Vec<KeyValue<Key,V>>,
  map: RegexMap<V>,
}

pub enum RegexMap<V> {
  Building(Vec<(Key, V)>),
  Map(RegexSet),
}

impl<V: Ord+Display> Machine<V> {
  pub fn new() -> Self {
    Machine {
      index: Vec::new(),
      map: RegexMap::Building(Vec::new()),
    }
  }

  pub fn finish(&mut self) {
    let mut set;

    match self.map {
      RegexMap::Map(_) => panic!("already finished"),
      RegexMap::Building(ref mut v) => {
        //v.sort_by(|a, b| a.0.iter().rev().cmp(b.0.iter().rev()));
        set = RegexSetBuilder::new(v.iter().map(|(k, v)| {
          let mut k = k.to_vec();
          k.reverse();
          String::from_utf8(k).unwrap()

        }));

        for (k, v) in v.drain(..) {
          self.index.push((k, v));
        }

        /*
        let mut index = 0u64;
        for (k, v) in v.drain(..) {
          let mut key = k.to_vec();
          key.reverse();

          if let Err(e) = builder.insert(&key, index) {
            //println!("error inserting key: {:?}", e);
          } else {
            //println!("inserted {} -> ({}, {})", std::str::from_utf8(&k).unwrap(),
            //  std::str::from_utf8(&key).unwrap(), v);
            self.index.push((k, v));
            index += 1;
          }
        }
        */
      }
    }

    set.size_limit(1048576 *200);
    self.map = RegexMap::Map(set.build().unwrap());
  }

  pub fn lookup(&self, key: &[u8]) -> Option<u64> {
    let mut partial_key = key.to_vec();
    partial_key.reverse();

    match self.map {
      RegexMap::Map(ref m) => {
        m.matches(&partial_key).into_iter().next().map(|i| i as u64)
      },
      RegexMap::Building(_) => {
        panic!("builder not finished");
      }
    }
  }
}

impl<V> DomainLookup<V> for Machine<V> {
  // specific version that will handle wildcard domains
  fn domain_insert(&mut self, key: Key, value: V) -> InsertResult {
    //let mut k = key.to_vec();
    //k.reverse();

    match self.map {
      RegexMap::Map(_) => panic!("already finished"),
      RegexMap::Building(ref mut v) => {
        v.push((key, value));
      }
    }

    InsertResult::Ok
  }

  // specific version that will handle wildcard domains
  fn domain_remove(&mut self, _key: &Key) -> RemoveResult {
    unimplemented!()
  }

  // specific version that will handle wildcard domains
  fn domain_lookup(&self, key: &[u8]) -> Option<&KeyValue<Key,V>> {
    let mut partial_key = key.to_vec();
    partial_key.reverse();

    //println!("looking up {} -> {}", std::str::from_utf8(key).unwrap(), std::str::from_utf8(&partial_key).unwrap());

    match self.map {
      RegexMap::Map(ref m) => {
        //println!("regexmap: {:?}", m);
        let r = m.matches(&partial_key);
        //println!("res: {:?}", r);
        r.into_iter().next().and_then(|i| self.index.get(i as usize))
      },
      RegexMap::Building(_) => {
        panic!("builder not finished");
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  /*
  #[test]
  fn insert() {
    let mut root:Machine<u8> = Machine::new();

    assert_eq!(root.insert(Vec::from(&b"abcd"[..]), 1), InsertResult::Ok);
    assert_eq!(root.insert(Vec::from(&b"abce"[..]), 2), InsertResult::Ok);
    assert_eq!(root.insert(Vec::from(&b"abgh"[..]), 3), InsertResult::Ok);

    root.finish();
    //assert_eq!(root.lookup(&b"abce"[..]), Some(&((&b"abce"[..]).to_vec(), 2)));
    //assert!(false);
  }

  #[test]
  fn remove() {
    let mut root: TrieNode<u8> = TrieNode::root();
    println!("creating root:");
    root.print();

    println!("adding (abcd, 1)");
    assert_eq!(root.insert(Vec::from(&b"abcd"[..]), 1), InsertResult::Ok);
    root.print();
    println!("adding (abce, 2)");
    assert_eq!(root.insert(Vec::from(&b"abce"[..]), 2), InsertResult::Ok);
    root.print();
    println!("adding (abgh, 3)");
    assert_eq!(root.insert(Vec::from(&b"abgh"[..]), 3), InsertResult::Ok);
    root.print();

    let mut root2: TrieNode<u8> = TrieNode::root();

    assert_eq!(root2.insert(Vec::from(&b"abcd"[..]), 1), InsertResult::Ok);
    assert_eq!(root2.insert(Vec::from(&b"abgh"[..]), 3), InsertResult::Ok);

    println!("before remove");
    root.print();
    assert_eq!(root.remove(&Vec::from(&b"abce"[..])), RemoveResult::Ok);
    println!("after remove");
    root.print();

    println!("expected");
    root2.print();
    assert_eq!(root, root2);

    assert_eq!(root.remove(&Vec::from(&b"abgh"[..])), RemoveResult::Ok);
    println!("after remove");
    root.print();
    println!("expected");
    let mut root3: TrieNode<u8> = TrieNode::root();
    assert_eq!(root3.insert(Vec::from(&b"abcd"[..]), 1), InsertResult::Ok);
    root3.print();
    assert_eq!(root, root3);
  }

  #[test]
  fn add_child_to_leaf() {
    let mut root1: TrieNode<u8> = TrieNode::root();

    println!("creating root1:");
    root1.print();
    println!("adding (abcd, 1)");
    assert_eq!(root1.insert(Vec::from(&b"abcd"[..]), 1), InsertResult::Ok);
    root1.print();
    println!("adding (abce, 2)");
    assert_eq!(root1.insert(Vec::from(&b"abce"[..]), 2), InsertResult::Ok);
    root1.print();
    println!("adding (abc, 3)");
    assert_eq!(root1.insert(Vec::from(&b"abc"[..]), 3), InsertResult::Ok);

    println!("root1:");
    root1.print();

    let mut root2: TrieNode<u8> = TrieNode::root();

    assert_eq!(root2.insert(Vec::from(&b"abc"[..]), 3), InsertResult::Ok);
    assert_eq!(root2.insert(Vec::from(&b"abcd"[..]), 1), InsertResult::Ok);
    assert_eq!(root2.insert(Vec::from(&b"abce"[..]), 2), InsertResult::Ok);

    println!("root2:");
    root2.print();
    assert_eq!(root2.remove(&Vec::from(&b"abc"[..])), RemoveResult::Ok);

    println!("root2 after,remove:");
    root2.print();
    let mut expected: TrieNode<u8> = TrieNode::root();

    assert_eq!(expected.insert(Vec::from(&b"abcd"[..]), 1), InsertResult::Ok);
    assert_eq!(expected.insert(Vec::from(&b"abce"[..]), 2), InsertResult::Ok);

    println!("root2 after insert");
    root2.print();
    println!("expected");
    expected.print();
    assert_eq!(root2, expected);
  }
  */

  #[test]
  fn domains() {
    let mut root: Machine<u8> = Machine::new();

    assert_eq!(root.domain_insert(Vec::from(&b"$www.example.com^"[..]), 1), InsertResult::Ok);
    assert_eq!(root.domain_insert(Vec::from(&b"$test.example.com^"[..]), 2), InsertResult::Ok);
    assert_eq!(root.domain_insert(Vec::from(&b"$*.alldomains.org^"[..]), 3), InsertResult::Ok);
    assert_eq!(root.domain_insert(Vec::from(&b"$alldomains.org^"[..]), 4), InsertResult::Ok);
    assert_eq!(root.domain_insert(Vec::from(&b"$hello.com^"[..]), 5), InsertResult::Ok);

    root.finish();

    assert_eq!(root.domain_lookup(&b"example.com"[..]), None);
    assert_eq!(root.domain_lookup(&b"blah.test.example.com"[..]), None);

    assert_eq!(root.domain_lookup(&b"www.example.com"[..]), Some(&((&b"$www.example.com^"[..]).to_vec(), 1)));
    //assert_eq!(root.domain_lookup(&b"alldomains.org"[..]), Some(&((&b"$alldomains.org^"[..]).to_vec(), 4)));
    assert_eq!(root.domain_lookup(&b"test.alldomains.org"[..]), Some(&((&b"$*.alldomains.org^"[..]).to_vec(), 3)));
    assert_eq!(root.domain_lookup(&b"hello.alldomains.org"[..]), Some(&((&b"$*.alldomains.org^"[..]).to_vec(), 3)));
    assert_eq!(root.domain_lookup(&b"blah.test.alldomains.org"[..]), None);

  }
}
