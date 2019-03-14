use std::{iter,str};
use std::fmt::Debug;

use super::{Key, KeyValue, InsertResult, RemoveResult, DomainLookup};
use super::cursor::*;

#[derive(Clone,Debug,PartialEq)]
pub struct TrieNode<V> {
  key_value: Option<KeyValue<Key, V>>,
  matcher: TrieNodeMatch<V>,
}

#[derive(Clone,Debug,PartialEq)]
enum TrieNodeMatch<V> {
  None,
  HostPrefix(TrieNodePrefix<V>),
  HostRegex(TrieNodeRegex<V>),
  HostWildCard(TrieNodeWildcard),
  PathPrefix(TrieNodePrefix<V>),
  PathRegex(TrieNodeRegex<V>),
}

#[derive(Clone,Debug,PartialEq)]
pub struct TrieNodePrefix<V> {
  local_key:  Key,
  child_keys: Vec<u8>,
  children:   Vec<TrieNode<V>>,
}

#[derive(Clone,Debug)]
pub struct TrieNodeRegex<V> {
  regexes: Vec<regex::bytes::Regex>,
  children: Vec<TrieNode<V>>,
}

impl<V: PartialEq> PartialEq for TrieNodeRegex<V> {
  fn eq(&self, other: &TrieNodeRegex<V>) -> bool {
    if self.children == other.children && self.regexes.len() == other.regexes.len() {
      for i in 0..self.regexes.len() {
        if self.regexes[i].as_str() != other.regexes[i].as_str() {
          return false;
        }
      }

      true
    } else {
      false
    }
  }
}

#[derive(Clone,Debug,PartialEq)]
pub struct TrieNodeWildcard;

impl<V:Debug> TrieNode<V> {
  /*pub fn new(key: Key, value: V) -> TrieNode<V> {
    TrieNode {
      key_value:  Some((key.clone(), value)),
      matcher: TrieNodeMatch::None,
    }
  }*/

  pub fn size(&self) -> usize {
    ::std::mem::size_of::<TrieNode<V>>()
    /*
    ::std::mem::size_of::<TrieNode<V>>() +
      ::std::mem::size_of::<Option<KeyValue<Key, V>>>()
      + self.local_key.len()
      + self.child_keys.len()
      + self.children.iter().fold(0, |acc, c| acc + c.size())
      */
  }

  pub fn root() -> TrieNode<V> {
    TrieNode {
      key_value: None,
      matcher: TrieNodeMatch::None,
    }
  }

  pub fn insert(&mut self, key: Key, value: V) -> InsertResult {
    unimplemented!();
    /*
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
    */
  }

  pub fn insert_recursive(&mut self, partial_key: &[u8], key: &Key, value: V) -> InsertResult {
    unimplemented!();
    //assert_ne!(partial_key, &b""[..]);

    /*println!("insert_recursive: partial_key={}, local_key={}",
      str::from_utf8(partial_key).unwrap(),
      str::from_utf8(&self.local_key).unwrap());
    */

    /*
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
    */
  }

  pub fn remove(&mut self, partial_key: &Key) -> RemoveResult {
    unimplemented!();
    /*println!("remove: partial_key={}, local_key={}",
      str::from_utf8(partial_key).unwrap(),
      str::from_utf8(&self.local_key).unwrap());
      */

    /*
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
    */

  }

  pub fn remove_recursive(&mut self, partial_key: &[u8]) -> RemoveResult {
    unimplemented!();

    /*
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

            let _k = self.child_keys.remove(0);
            let mut child = self.children.remove(0);
            self.local_key.extend(child.local_key.drain(..));
            if !child.child_keys.is_empty() {
              self.child_keys = child.child_keys;
              self.children = child.children;
            }
          }

        }

        res
      }
    }
    */
  }

  // specific version that will handle wildcard domains
  pub fn domain_lookup_recursive(&self, partial_key: &[u8]) -> Option<&KeyValue<Key,V>> {
    unimplemented!();
    //assert_ne!(partial_key, &b""[..]);

    /*println!("domain_lookup_recursive: partial_key={}, local_key={}, child_keys={}",
      str::from_utf8(partial_key).unwrap(),
      str::from_utf8(&self.local_key).unwrap(),
      str::from_utf8(&self.child_keys).unwrap(),
    );
    */

    /*
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
    */
  }

  pub fn print(&self) {
    self.print_recursive(b'.', 0)
  }

  pub fn print_recursive(&self, partial_key: u8, indent:u8) {
    unimplemented!();
    /*
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
    */
  }
}

impl<V: Debug> DomainLookup<V> for TrieNode<V> {
  // specific version that will handle wildcard domains
  fn domain_insert(&mut self, key: Key, value: V) -> InsertResult {
    unimplemented!();
    /*
    let mut partial_key = key.clone();
    partial_key.reverse();

    //handle the root
    if self.local_key.is_empty() && self.child_keys.is_empty() {
      self.local_key = partial_key;
      self.key_value = Some((key, value));
      return InsertResult::Ok;
    }

    self.insert_recursive(&partial_key, &key, value)
      */
  }

  // specific version that will handle wildcard domains
  fn domain_remove(&mut self, key: &Key) -> RemoveResult {
    unimplemented!();
    /*
    let mut partial_key = key.clone();
    partial_key.reverse();
    self.remove(&partial_key)
    */
  }

  // specific version that will handle wildcard domains
  fn domain_lookup(&self, key: &[u8]) -> Option<&KeyValue<Key,V>> {
    unimplemented!();
    /*
    let mut partial_key = key.to_vec();
    partial_key.reverse();
    self.domain_lookup_recursive(&partial_key)
    */
  }
}
