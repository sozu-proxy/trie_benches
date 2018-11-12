//! this experiment builds upon experiment 1, by separating the child_keys from the nodes.
//! Now, instead of a single `Vec<(Key, TrieNode<V>)>`, there's a `Vec<Key>` and
//! a `Vec<TrieNode<V>>`

use std::{iter,str};
use std::fmt::Debug;
use rand::XorShiftRng;

use gen_seed::{gen_uuid_seed_domain, gen_text_seed_domain, gen_seed_wilcard_domain};

pub type Key = Vec<u8>;
pub type KeyValue<K,V> = (K,V);

#[derive(Debug,PartialEq)]
pub struct TrieNode<V> {
  key_value:  Option<KeyValue<Key,V>>,
  local_key:  Key,
  child_keys: Vec<u8>,
  children:   Vec<TrieNode<V>>,
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

impl<V:Debug> TrieNode<V> {
  pub fn new(key: Key, value: V) -> TrieNode<V> {
    TrieNode {
      key_value:  Some((key.clone(), value)),
      local_key:  key,
      child_keys: vec!(),
      children:   vec!(),
    }
  }

  pub fn root() -> TrieNode<V> {
    TrieNode {
      key_value:  None,
      local_key:  vec!(),
      child_keys: vec!(),
      children:   vec!(),
    }
  }

  pub fn insert(&mut self, key: Key, value: V) -> InsertResult {
    //handle the root
    if self.local_key.is_empty() && self.child_keys.is_empty() {
      self.local_key = key.clone();
      self.key_value = Some((key, value));
      return InsertResult::Ok;
    }

    let res = self.insert_recursive(&key, &key, value);
    assert_ne!(res, InsertResult::Failed);
    //println!("adding {}", str::from_utf8(&key).unwrap());
    res
  }

  pub fn insert_recursive(&mut self, partial_key: &[u8], key: &Key, value: V) -> InsertResult {
    assert_ne!(partial_key, &b""[..]);

    /*println!("insert_recursive: partial_key={}, local_key={}",
      str::from_utf8(partial_key).unwrap(),
      str::from_utf8(&self.local_key).unwrap());
    */

    let pos = partial_key.iter().zip(self.local_key.iter()).position(|(&a,&b)| a != b);
    match pos {
      None => {
        if partial_key.len() > self.local_key.len() {
          //match self.child_keys.iter().position(|k| *k == partial_key[self.local_key.len()]) {
          match self.child_keys.iter().position(|k| *k == partial_key[self.local_key.len()]) {
            None => {
              let new_child = TrieNode {
                key_value:  Some((key.clone(), value)),
                local_key:  partial_key[self.local_key.len()..].to_vec(),
                child_keys: vec!(),
                children:   vec!(),
              };
              self.child_keys.push(partial_key[self.local_key.len()]);
              self.children.push(new_child);

              return InsertResult::Ok;
            }
            Some(index) => {
              return self.children[index].insert_recursive(&partial_key[self.local_key.len()..], key, value);
            }
          }
        } else if partial_key.len() == self.local_key.len() {
          if self.key_value.is_some() {
            return InsertResult::Existing;
          } else {
            self.key_value =  Some((key.clone(), value));
          }
        } else {
          //partial key is smaller, so insert the new value above
          //the current node
          let new_child = TrieNode {
            key_value:  self.key_value.take(),
            local_key:  self.local_key[partial_key.len()..].to_vec(),
            child_keys: vec!(),
            children:   vec!(),
          };

          self.key_value =  Some((key.clone(), value));
          self.child_keys.push(self.local_key[partial_key.len()]);
          self.children.push(new_child);
          self.local_key = partial_key.to_vec();

          return InsertResult::Ok;

        }
      },
      Some(index) => {
        let new_child1 = TrieNode {
          key_value:  self.key_value.take(),
          local_key:  self.local_key[index..].to_vec(),
          child_keys: self.child_keys.drain(..).collect(),
          children:   self.children.drain(..).collect(),
        };
        let new_child2 = TrieNode {
          key_value:  Some((key.clone(), value)),
          local_key:  partial_key[index..].to_vec(),
          child_keys: vec!(),
          children:   vec!(),
        };

        self.child_keys.push(self.local_key[index]);
        self.children.push(new_child1);
        self.child_keys.push(partial_key[index]);
        self.children.push(new_child2);
        self.local_key.truncate(index);
        return InsertResult::Ok;
      }
    }

    return InsertResult::Ok;
  }

  pub fn remove(&mut self, partial_key: &Key) -> RemoveResult {
    /*println!("remove: partial_key={}, local_key={}",
      str::from_utf8(partial_key).unwrap(),
      str::from_utf8(&self.local_key).unwrap());
      */

    //we check the lower level's local_key in remove_recursive,
    //so we handle the root node here
    match partial_key.iter().zip(self.local_key.iter()).position(|(&a,&b)| a != b) {
      None => {
        let local_len = self.local_key.len();
        if partial_key.len() > local_len {
          self.remove_recursive(&partial_key[local_len..])
        } else if partial_key.len() == local_len {
          if self.key_value.is_some() {
            self.key_value = None;
            if self.child_keys.is_empty() {
              self.local_key = vec!();
            }

            RemoveResult::Ok
          } else {
            RemoveResult::NotFound
          }
        } else {
          RemoveResult::NotFound
        }
      },
      Some(_) => RemoveResult::NotFound
    }

  }

  pub fn remove_recursive(&mut self, partial_key: &[u8]) -> RemoveResult {
    /*println!("remove_recursive: partial_key={}, local_key={}, child_keys={}",
      str::from_utf8(partial_key).unwrap(),
      str::from_utf8(&self.local_key).unwrap(),
      str::from_utf8(&self.child_keys).unwrap());
    */

    assert_ne!(partial_key, &b""[..]);
    match self.child_keys.iter().position(|k| *k == partial_key[0]) {
      None => RemoveResult::NotFound,
      Some(index) => {
        let res = {
          let child = &mut self.children[index];
          match partial_key.iter().zip(child.local_key.iter()).position(|(&a,&b)| a != b) {
            None => {
              let child_local_len = child.local_key.len();
              if partial_key.len() > child_local_len {
                child.remove_recursive(&partial_key[child_local_len..])
              } else if partial_key.len() == child_local_len {
                if child.key_value.is_some() {
        //println!("removing key_value: {:?}", child.key_value);
                  child.key_value = None;
                  RemoveResult::Ok
                } else {
                  RemoveResult::NotFound
                }
              } else {
                RemoveResult::NotFound
              }
            },
            Some(_) => RemoveResult::NotFound
          }
        };

        // we might have some cleanup to do
        if res == RemoveResult::Ok {
          /*println!("will cleanup. child has key_value:{} child has child_keys:{}",
          self.children[index].key_value.is_some(),
          self.children[index].child_keys.len());
          self.print();
          */


          if self.children[index].key_value.is_none() && self.children[index].child_keys.is_empty() {
            self.child_keys.remove(index);
            self.children.remove(index);
          }

          if self.child_keys.len() == 1 {
            match (self.key_value.is_some(), self.children[0].key_value.is_some()) {
              // we keep the child because both current node and child have values
              (true, true) => return res,
              // we take the child's value
              (false, true) => {
                self.key_value = self.children[0].key_value.take();
              },
              _ => { }
            }

            let k = self.child_keys.remove(0);
            let mut child = self.children.remove(0);
            self.local_key.extend(child.local_key.drain(..));
            if !child.child_keys.is_empty() {
              self.child_keys = child.child_keys;
              self.children = child.children;
            }
          }

        /*
        println!("remove_recursive: after cleanup partial_key={}, local_key={}, child_keys={}",
      str::from_utf8(partial_key).unwrap(),
      str::from_utf8(&self.local_key).unwrap(),
      str::from_utf8(&self.child_keys).unwrap());
          self.print();
          */

        }

        res
      }
    }
  }

  // specific version that will handle wildcard domains
  pub fn domain_insert(&mut self, key: Key, value: V) -> InsertResult {
    let mut partial_key = key.clone();
    partial_key.reverse();

    //handle the root
    if self.local_key.is_empty() && self.child_keys.is_empty() {
      self.local_key = partial_key;
      self.key_value = Some((key, value));
      return InsertResult::Ok;
    }

    self.insert_recursive(&partial_key, &key, value)
  }

  // specific version that will handle wildcard domains
  pub fn domain_remove(&mut self, key: &Key) -> RemoveResult {
    let mut partial_key = key.clone();
    partial_key.reverse();
    self.remove(&partial_key)
  }

  // specific version that will handle wildcard domains
  pub fn domain_lookup(&self, key: &[u8]) -> Option<&KeyValue<Key,V>> {
    let mut partial_key = key.to_vec();
    partial_key.reverse();
    self.domain_lookup_recursive(&partial_key)
  }

  // specific version that will handle wildcard domains
  pub fn domain_lookup_recursive(&self, partial_key: &[u8]) -> Option<&KeyValue<Key,V>> {
    assert_ne!(partial_key, &b""[..]);

    /*println!("domain_lookup_recursive: partial_key={}, local_key={}, child_keys={}",
      str::from_utf8(partial_key).unwrap(),
      str::from_utf8(&self.local_key).unwrap(),
      str::from_utf8(&self.child_keys).unwrap(),
    );
    */
    let pos = partial_key.iter().zip(self.local_key.iter()).position(|(&a,&b)| a != b);
    match pos {
      None => {
        let local_len = self.local_key.len();
        if partial_key.len() > local_len {
          match self.child_keys.iter().position(|k| *k == partial_key[local_len]) {
            None => None,
            Some(index) => {
              self.children[index].domain_lookup_recursive(&partial_key[local_len..])
            }
          }
        } else if partial_key.len() == local_len {
          self.key_value.as_ref()
        } else {
          None
        }
      },
      Some(i) => {
        // check for wildcard
        if i+1 == self.local_key.len() && self.local_key[i] == '*' as u8 {
          let c = '.' as u8;
          if (&partial_key[i..]).contains(&c) {
            None
          } else {
            self.key_value.as_ref()
          }
        } else {
          None
        }

      }
    }
  }

  pub fn print(&self) {
    self.print_recursive(b'.', 0)
  }

  pub fn print_recursive(&self, partial_key: u8, indent:u8) {
    let raw_prefix:Vec<u8> = iter::repeat(' ' as u8).take(2*indent as usize).collect();
    let prefix = str::from_utf8(&raw_prefix).unwrap();
    let c: char = partial_key.into();

    if let Some((ref key, ref value)) = self.key_value {
    println!("{}{}: {}|({},{:?})", prefix, c,
      str::from_utf8(&self.local_key).unwrap(),
      str::from_utf8(&key).unwrap(), value);
    } else {
    println!("{}{}: {}|None", prefix, c,
      str::from_utf8(&self.local_key).unwrap());
    }
    for (child_key, ref child) in self.child_keys.iter().zip(self.children.iter()) {
      child.print_recursive(*child_key, indent+1);
    }
  }
}

/// Feed a seed trie with: (nb_elems_seed)
/// 1/3 uui.uuid.tld
/// 1/3 domain_text.uuid.tld
/// 1/3 *.uuid.tld
pub fn seed_bench_trie(root: &mut TrieNode<u8>, nb_elems_seed: i32) {
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

  #[test]
  fn domains() {
    let mut root: TrieNode<u8> = TrieNode::root();
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
