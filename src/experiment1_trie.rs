//! this example modifies sozu's original trie implementation to move the partial keys
//! in the parent node

use std::{iter,str};
use std::fmt::Debug;

use super::{Key, KeyValue, InsertResult, RemoveResult, DomainLookup};

#[derive(Debug,PartialEq)]
pub struct TrieNode<V> {
  key_value:   Option<KeyValue<Key,V>>,
  children:    Vec<(Key, TrieNode<V>)>,
}

impl<V:Debug> TrieNode<V> {
  pub fn new(key: Key, value: V) -> TrieNode<V> {
    TrieNode {
      key_value:      Some((key, value)),
      children:       vec!(),
    }
  }

  pub fn root() -> TrieNode<V> {
    TrieNode {
      key_value:      None,
      children:       vec!(),
    }
  }

  pub fn insert(&mut self, key: Key, value: V) -> InsertResult {
    let res = self.insert_recursive(&key, &key, value);
    assert_ne!(res, InsertResult::Failed);
    res
  }

  pub fn insert_recursive(&mut self, partial_key: &[u8], key: &Key, value: V) -> InsertResult {
    assert_ne!(partial_key, &b""[..]);

    let mut found = None;
    for (index, &(ref child_key, _)) in self.children.iter().enumerate() {
      let pos = partial_key.iter().zip(child_key.iter()).position(|(&a,&b)| a != b);
      match pos {
        Some(0) => continue,
        _       => found = Some(index),
      }
    }

    match found {
      None => {
        let new_child = TrieNode {
          key_value:   Some((key.clone(), value)),
          children:    vec!(),
        };
        self.children.push((partial_key.to_vec(), new_child));
      },
      Some(index) => {
        let (child_key, mut child) = self.children.remove(index);
        let pos = partial_key.iter().zip(child_key.iter()).position(|(&a,&b)| a != b);
        match pos {
          Some(i) => {
            let new_child = TrieNode {
              key_value:   Some((key.clone(), value)),
              children:    vec!(),
            };

            let new_parent = TrieNode {
              key_value: None,
              children: vec!(
                ((&child_key[i..]).to_vec(), child),
                ((&partial_key[i..]).to_vec(), new_child)
              ),
            };

            self.children.push(((&partial_key[..i]).to_vec(), new_parent));
          },
          None => {
          if partial_key.len() > child_key.len()  {
            let i = child_key.len();
            let res = child.insert_recursive(&partial_key[i..], key, value);
            self.children.push(((&partial_key[..i]).to_vec(), child));
            return res;
          } else if partial_key.len() == child_key.len() {
            if child.key_value.is_some() {
              return InsertResult::Existing;
            } else {
              child.key_value = Some((key.clone(), value));
              return InsertResult::Ok;
            }
          } else {
            // the partial key is smaller, insert as parent
            let i = partial_key.len();
            let new_parent = TrieNode {
              key_value: Some((key.clone(), value)),
              children: vec!(
                ((&child_key[i..]).to_vec(), child),
              ),
            };
            self.children.push((partial_key.to_vec(), new_parent));

            return InsertResult::Ok;
          }

          }
        }

      }
    }

    return InsertResult::Ok;
  }

  pub fn remove(&mut self, key: &Key) -> RemoveResult {
    self.remove_recursive(key)
  }

  pub fn remove_recursive(&mut self, partial_key: &[u8]) -> RemoveResult {
    assert_ne!(partial_key, &b""[..]);

    let mut found = None;

    for (index, &mut (ref child_key, ref mut child)) in self.children.iter_mut().enumerate() {
      let pos = partial_key.iter().zip(child_key.iter()).position(|(&a,&b)| a != b);
      match pos {
        Some(_) => continue,
        None    => {
          if partial_key.len() > child_key.len()  {
            let i = child_key.len();
            return child.remove_recursive(&partial_key[i..]);
          } else if partial_key.len() == child_key.len() {
            found = Some(index);
            break;
          } else {
            continue
          }
        }
      };

    }

    if let Some(index) = found {
      if self.children[index].1.children.len() > 0 && self.children[index].1.key_value.is_some() {
        self.children[index].1.key_value = None;
        return RemoveResult::Ok;
      } else {
        self.children.remove(index);

        // we might get into a case where there's only one child, and we could merge it
        // with the parent, but this case is not handle right now
        return RemoveResult::Ok;
      }
    } else {
      RemoveResult::NotFound
    }
  }

  /*
  pub fn lookup(&self, partial_key: &[u8]) -> Option<&KeyValue<Key,V>> {
    assert_ne!(partial_key, &b""[..]);

    if partial_key.starts_with(&self.partial_key) {
      if partial_key.len() == self.partial_key.len() {
        return self.key_value.as_ref();
      } else {
        for &(ref child_key, ref child) in self.children.iter() {
          let res = child.lookup(&partial_key[child_key.len()..]);
          if res.is_some() {
            return res
          }
        }
        None
      }
    } else {
      None
    }
  }
  */

  // specific version that will handle wildcard domains
  pub fn domain_lookup_recursive(&self, partial_key: &[u8]) -> Option<&KeyValue<Key,V>> {
    assert_ne!(partial_key, &b""[..]);

    for &(ref child_key, ref child) in self.children.iter() {
      let pos = partial_key.iter().zip(child_key.iter()).position(|(&a,&b)| a != b);
      match pos {
        Some(0) => continue,
        Some(i) => {
          // check for wildcard
          if i+1 == child_key.len() && child_key[i] == '*' as u8 {
            let c = '.' as u8;
            if (&partial_key[i..]).contains(&c) {
              return None;
            } else {
              return child.key_value.as_ref();
            }
          } else {
            return None;
          }
        },
        None    => {
          if partial_key.len() > child_key.len() {
            return child.domain_lookup_recursive(&partial_key[child_key.len()..]);
          } else if partial_key.len() == child_key.len() {
            return child.key_value.as_ref();
          } else {
            return None;
          }
        }
      }
    }

    None
  }

  pub fn print(&self) {
    self.print_recursive(b"", 0)
  }

  pub fn print_recursive(&self, partial_key: &[u8], indent:u8) {
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
  }
}

impl<V: Debug> DomainLookup<V> for TrieNode<V> {

  // specific version that will handle wildcard domains
  fn domain_insert(&mut self, key: Key, value: V) -> InsertResult {
    let mut partial_key = key.clone();
    partial_key.reverse();
    self.insert_recursive(&partial_key, &key, value)
  }

  // specific version that will handle wildcard domains
  fn domain_remove(&mut self, key: &Key) -> RemoveResult {
    let mut partial_key = key.clone();
    partial_key.reverse();
    self.remove_recursive(&partial_key)
  }

  // specific version that will handle wildcard domains
  fn domain_lookup(&self, key: &[u8]) -> Option<&KeyValue<Key,V>> {
    let mut partial_key = key.to_vec();
    partial_key.reverse();
    self.domain_lookup_recursive(&partial_key)
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
