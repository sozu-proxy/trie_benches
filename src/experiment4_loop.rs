//! this experiment builds upon experiment 1, by separating the child_keys from the nodes.
//! Now, instead of a single `Vec<(Key, List<V>)>`, there's a `Vec<Key>` and
//! a `Vec<List<V>>`

use std::{iter,str};
use std::fmt::Debug;
use rand::XorShiftRng;

use gen_seed::{gen_uuid_seed_domain, gen_text_seed_domain, gen_seed_wilcard_domain};

pub type Key = Vec<u8>;
pub type KeyValue<K,V> = (K,V);


#[derive(Debug,PartialEq)]
pub struct List<V> {
  pub acl : Vec<(Vec<u8>, KeyValue<Key, V>)>,
}

#[derive(Debug,PartialEq)]
pub enum InsertResult {
  Ok,
  Existing,
  Failed
}

#[derive(Debug,PartialEq)]
pub enum RemoveResult {
  Ok,
  NotFound,
}

impl<V:Debug> List<V> {

  pub fn new() -> List<V> {
    List {
      acl: Vec::new(),
    }
  }

  pub fn root() -> List<V> {
    List {
      acl: Vec::new(),
    }
  }

  pub fn insert(&mut self, key: Key, value: V) -> InsertResult {
    self.acl.push((key.clone(), (key, value)));
    InsertResult::Ok
  }

  // specific version that will handle wildcard domains
  pub fn domain_insert(&mut self, key: Key, value: V) -> InsertResult {
    let mut partial_key = key.clone();
    partial_key.reverse();
    self.acl.push((partial_key, (key, value)));
    InsertResult::Ok
  }

  // specific version that will handle wildcard domains
  pub fn domain_remove(&mut self, key: &Key) -> RemoveResult {
    let mut partial_key = key.clone();
    partial_key.reverse();
    match self.acl.iter().position(|v| v.0 == partial_key) {
      Some(pos) => {
        self.acl.remove(pos);
        RemoveResult::Ok
      },
      None => RemoveResult::NotFound
    }
    //let mut partial_key = key.clone();
    //partial_key.reverse();
    //self.acl.remove(&partial_key)
    //panic!();
  }

  // specific version that will handle wildcard domains
  pub fn domain_lookup(&self, key: &[u8]) -> Option<&KeyValue<Key,V>> {
    let mut partial_key = key.to_vec();
    partial_key.reverse();

    for (local_key, key_value) in self.acl.iter() {
      let pos = partial_key.iter().zip(local_key.iter()).position(|(&a,&b)| a != b);
      match pos {
        None => {
          let local_len = local_key.len();
          if partial_key.len() == local_len {
            return Some(key_value);
          }
        },
        Some(i) => {
          // check for wildcard
          if i+1 == local_key.len() && local_key[i] == '*' as u8 {
            let c = '.' as u8;
            if !(&partial_key[i..]).contains(&c) {
              return Some(key_value);
            }
          }
        }
      }
    }

    None
  }

  pub fn print(&self) {
    println!("{:?}", self);
  }
}

/// Feed a seed trie with: (nb_elems_seed)
/// 1/3 uui.uuid.tld
/// 1/3 domain_text.uuid.tld
/// 1/3 *.uuid.tld
pub fn seed_bench_trie(root: &mut List<u8>, nb_elems_seed: i32) {
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

#[cfg(test)]
mod tests {
  use super::*;

  /*
  #[test]
  fn insert() {
    let mut root: List<u8> = List::root();
    root.print();

    assert_eq!(root.insert(Vec::from(&b"abcd"[..]), 1), InsertResult::Ok);
    root.print();
    assert_eq!(root.insert(Vec::from(&b"abce"[..]), 2), InsertResult::Ok);
    root.print();
    assert_eq!(root.insert(Vec::from(&b"abgh"[..]), 3), InsertResult::Ok);
    root.print();

    //assert_eq!(root.lookup(&b"abce"[..]), Some(&((&b"abce"[..]).to_vec(), 2)));
    //assert!(false);
  }

  #[test]
  fn remove() {
    let mut root: List<u8> = List::root();
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

    let mut root2: List<u8> = List::root();

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
    let mut root3: List<u8> = List::root();
    assert_eq!(root3.insert(Vec::from(&b"abcd"[..]), 1), InsertResult::Ok);
    root3.print();
    assert_eq!(root, root3);
  }

  #[test]
  fn add_child_to_leaf() {
    let mut root1: List<u8> = List::root();

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

    let mut root2: List<u8> = List::root();

    assert_eq!(root2.insert(Vec::from(&b"abc"[..]), 3), InsertResult::Ok);
    assert_eq!(root2.insert(Vec::from(&b"abcd"[..]), 1), InsertResult::Ok);
    assert_eq!(root2.insert(Vec::from(&b"abce"[..]), 2), InsertResult::Ok);

    println!("root2:");
    root2.print();
    assert_eq!(root2.remove(&Vec::from(&b"abc"[..])), RemoveResult::Ok);

    println!("root2 after,remove:");
    root2.print();
    let mut expected: List<u8> = List::root();

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
    let mut root: List<u8> = List::root();
    root.print();

    assert_eq!(root.domain_insert(Vec::from(&b"www.example.com"[..]), 1), InsertResult::Ok);
    root.print();
    assert_eq!(root.domain_insert(Vec::from(&b"test.example.com"[..]), 2), InsertResult::Ok);
    root.print();
    assert_eq!(root.domain_insert(Vec::from(&b"*.alldomains.org"[..]), 3), InsertResult::Ok);
    root.print();
    assert_eq!(root.domain_insert(Vec::from(&b"alldomains.org"[..]), 4), InsertResult::Ok);
    root.print();
    assert_eq!(root.domain_insert(Vec::from(&b"hello.com"[..]), 5), InsertResult::Ok);
    root.print();

    assert_eq!(root.domain_lookup(&b"example.com"[..]), None);
    assert_eq!(root.domain_lookup(&b"blah.test.example.com"[..]), None);
    assert_eq!(root.domain_lookup(&b"www.example.com"[..]), Some(&((&b"www.example.com"[..]).to_vec(), 1)));
    assert_eq!(root.domain_lookup(&b"alldomains.org"[..]), Some(&((&b"alldomains.org"[..]).to_vec(), 4)));
    assert_eq!(root.domain_lookup(&b"test.alldomains.org"[..]), Some(&((&b"*.alldomains.org"[..]).to_vec(), 3)));
    assert_eq!(root.domain_lookup(&b"hello.alldomains.org"[..]), Some(&((&b"*.alldomains.org"[..]).to_vec(), 3)));
    assert_eq!(root.domain_lookup(&b"blah.test.alldomains.org"[..]), None);

    assert_eq!(root.domain_remove(&Vec::from(&b"alldomains.org"[..])), RemoveResult::Ok);
    println!("after remove");
    root.print();
    assert_eq!(root.domain_lookup(&b"alldomains.org"[..]), None);
    assert_eq!(root.domain_lookup(&b"test.alldomains.org"[..]), Some(&((&b"*.alldomains.org"[..]).to_vec(), 3)));
    assert_eq!(root.domain_lookup(&b"hello.alldomains.org"[..]), Some(&((&b"*.alldomains.org"[..]).to_vec(), 3)));
    assert_eq!(root.domain_lookup(&b"blah.test.alldomains.org"[..]), None);
  }
}
