//! this example modifies sozu's original trie implementation to move the partial keys
//! in the parent node

use std::{iter,str};
use std::fmt::Debug;

use super::{Key, KeyValue, InsertResult, RemoveResult, DomainLookup};
use hashbrown::HashMap;

fn find_last_dot(input: &[u8]) -> Option<usize> {
  ////println!("find_last_dot: input = {}", from_utf8(input).unwrap());
  for i in (0..input.len()).rev() {
    ////println!("input[{}] -> {}", i, input[i] as char);
    if input[i] == b'.' {
      return Some(i);
    }
  }

  None
}

#[derive(Debug,PartialEq)]
pub struct TrieNode<V> {
  key_value: Option<KeyValue<Key,V>>,
  children:  HashMap<Key, TrieNode<V>>,
}

impl<V:Debug> TrieNode<V> {
  pub fn new(key: Key, value: V) -> TrieNode<V> {
    TrieNode {
      key_value: Some((key, value)),
      children:  HashMap::new(),
    }
  }

  pub fn root() -> TrieNode<V> {
    TrieNode {
      key_value: None,
      children:  HashMap::new(),
    }
  }

  pub fn insert(&mut self, key: Key, value: V) -> InsertResult {
    let res = self.insert_recursive(&key, &key, value);
    assert_ne!(res, InsertResult::Failed);
    res
  }

  pub fn insert_recursive(&mut self, partial_key: &[u8], key: &Key, value: V) -> InsertResult {
    //println!("insert: key == {}", std::str::from_utf8(partial_key).unwrap());
    assert_ne!(partial_key, &b""[..]);

    let pos = find_last_dot(partial_key);
    match pos {
      None => {
        if self.children.contains_key(partial_key) {
          InsertResult::Existing
        } else {
          let node = TrieNode::new(partial_key.to_vec(), value);
          self.children.insert(partial_key.to_vec(), node);
          InsertResult::Ok
        }
      }
      Some(pos) => {
        if let Some(child) = self.children.get_mut(&partial_key[pos..]) {
          return child.insert_recursive(&partial_key[..pos], key, value);
        }

        let mut node = TrieNode::root();
        node.insert_recursive(&partial_key[..pos], key, value);
        self.children.insert((&partial_key[pos..]).to_vec(), node);
        InsertResult::Ok
        /*if let Some(child) = self.children.get_mut(&partial_key[partial_key.len() - pos - 1..]) {
          return child.insert_recursive(&partial_key[..partial_key.len() - pos - 1], key, value);
        }

        let node = TrieNode::new(partial_key.to_vec(), value);
        self.children.insert((&partial_key[partial_key.len() - pos - 1..]).to_vec(), node);
        InsertResult::Ok
          */
      }
    }

  }

  pub fn remove(&mut self, key: &Key) -> RemoveResult {
    self.remove_recursive(key)
  }

  pub fn remove_recursive(&mut self, partial_key: &[u8]) -> RemoveResult {
    unimplemented!()
  }

  pub fn lookup(&self, partial_key: &[u8]) -> Option<&KeyValue<Key,V>> {
    //println!("lookup: key == {}", std::str::from_utf8(partial_key).unwrap());

    if partial_key.len() == 0 {
      return self.key_value.as_ref();
    }

    let pos = find_last_dot(partial_key);
    let (prefix, suffix) = match pos {
      None => (&b""[..], partial_key),
      //Some(pos) => (&partial_key[..partial_key.len() - pos - 1], &partial_key[partial_key.len() - pos - 1..]),
      Some(pos) => (&partial_key[..pos], &partial_key[pos..]),
    };
    //println!("lookup: prefix|suffix: {} | {}", std::str::from_utf8(prefix).unwrap(), std::str::from_utf8(suffix).unwrap());

    self.children.get(suffix).and_then(|child| child.lookup(prefix))
  }

  pub fn print(&self) {
    self.print_recursive(b"", 0)
  }

  pub fn print_recursive(&self, partial_key: &[u8], indent:u8) {
    /*
    let raw_prefix:Vec<u8> = iter::repeat(' ' as u8).take(2*indent as usize).collect();
    let prefix = str::from_utf8(&raw_prefix).unwrap();

    if let Some((ref key, ref value)) = self.key_value {
    println!("{}{}: ({},{:?})", prefix, str::from_utf8(partial_key).unwrap(),
      str::from_utf8(&key).unwrap(), value);
    } else {
    println!("{}{}: None", prefix, str::from_utf8(partial_key).unwrap());
    }
    for &(ref child_key, ref child) in self.children.iter() {
      child.print_recursive(child_key, indent+1);
    }
    */
  }
}

impl<V: Debug> DomainLookup<V> for TrieNode<V> {

  // specific version that will handle wildcard domains
  fn domain_insert(&mut self, key: Key, value: V) -> InsertResult {
    self.insert(key, value)
    /*
    let mut partial_key = key.clone();
    partial_key.reverse();
    self.insert_recursive(&partial_key, &key, value)
      */
  }

  // specific version that will handle wildcard domains
  fn domain_remove(&mut self, key: &Key) -> RemoveResult {
    unimplemented!()
    /*
    let mut partial_key = key.clone();
    partial_key.reverse();
    self.remove_recursive(&partial_key)
      */
  }

  // specific version that will handle wildcard domains
  fn domain_lookup(&self, key: &[u8]) -> Option<&KeyValue<Key,V>> {
    self.lookup(key)
    /*
    let mut partial_key = key.to_vec();
    partial_key.reverse();
    self.domain_lookup_recursive(&partial_key)
      */

  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn insert() {
    let mut root: TrieNode<u8> = TrieNode::root();
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
    let mut root: TrieNode<u8> = TrieNode::root();

    assert_eq!(root.insert(Vec::from(&b"abcd"[..]), 1), InsertResult::Ok);
    assert_eq!(root.insert(Vec::from(&b"abce"[..]), 2), InsertResult::Ok);
    assert_eq!(root.insert(Vec::from(&b"abgh"[..]), 3), InsertResult::Ok);

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
    let mut root: TrieNode<u8> = TrieNode::root();

    assert_eq!(root.insert(Vec::from(&b"abcd"[..]), 1), InsertResult::Ok);
    assert_eq!(root.insert(Vec::from(&b"abce"[..]), 2), InsertResult::Ok);
    assert_eq!(root.insert(Vec::from(&b"abc"[..]), 3), InsertResult::Ok);

    root.print();

    println!("ROOT 2:");
    let mut root2: TrieNode<u8> = TrieNode::root();

    root2.print();
    assert_eq!(root2.insert(Vec::from(&b"abc"[..]), 3), InsertResult::Ok);
    root2.print();
    assert_eq!(root2.insert(Vec::from(&b"abcd"[..]), 1), InsertResult::Ok);
    root2.print();
    assert_eq!(root2.insert(Vec::from(&b"abce"[..]), 2), InsertResult::Ok);

    root2.print();
    assert_eq!(root2.remove(&Vec::from(&b"abc"[..])), RemoveResult::Ok);

    let mut expected: TrieNode<u8> = TrieNode::root();

    assert_eq!(expected.insert(Vec::from(&b"abcd"[..]), 1), InsertResult::Ok);
    assert_eq!(expected.insert(Vec::from(&b"abce"[..]), 2), InsertResult::Ok);

    println!("after remove");
    root2.print();
    println!("expected");
    expected.print();
    assert_eq!(root2, expected);
  }

  #[test]
  fn domains() {
    let mut root: TrieNode<u8> = TrieNode::root();

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

    println!("STARTING LOOKUPS\n");
    println!("tree:\n{:#?}", root);

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
