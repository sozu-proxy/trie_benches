use std::{iter,str};
use std::fmt::Debug;
use rand::XorShiftRng;

use gen_seed::{gen_uuid_seed_domain, gen_text_seed_domain, gen_seed_wilcard_domain};

pub type Key = Vec<u8>;
pub type KeyValue<K,V> = (K,V);

#[derive(Debug,PartialEq)]
pub struct TrieNode<V> {
  partial_key: Key,
  key_value:   Option<KeyValue<Key,V>>,
  children:    Vec<TrieNode<V>>,
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
  pub fn new(partial: &[u8], key: Key, value: V) -> TrieNode<V> {
    TrieNode {
      partial_key:    Vec::from(partial),
      key_value:      Some((key, value)),
      children:       vec!(),
    }
  }

  pub fn root() -> TrieNode<V> {
    TrieNode {
      partial_key:    vec!(),
      key_value:      None,
      children:       vec!(),
    }
  }

  pub fn split(&mut self, index:usize) {
    let key_value = self.key_value.take();
    let children  = self.children.drain(..).collect();

    let child = TrieNode {
      partial_key: (&self.partial_key[index..]).to_vec(),
      key_value:   key_value,
      children:    children,
    };

    self.children.push(child);
    self.partial_key = (&self.partial_key[..index]).to_vec();
  }

  pub fn insert(&mut self, key: Key, value: V) -> InsertResult {
    let res = self.insert_recursive(&key, &key, value);
    assert_ne!(res, InsertResult::Failed);
    //println!("adding {}", str::from_utf8(&key).unwrap());
    res
  }

  pub fn insert_recursive(&mut self, partial_key: &[u8], key: &Key, value: V) -> InsertResult {
    assert_ne!(partial_key, &b""[..]);

    // checking directly the children
    for child in self.children.iter_mut() {
      let pos = partial_key.iter().zip(child.partial_key.iter()).position(|(&a,&b)| a != b);
      match pos {
        Some(0) => continue,
        Some(i) => {
          child.split(i);
          let new_child = TrieNode {
            partial_key: (&partial_key[i..]).to_vec(),
            key_value:   Some((key.clone(), value)),
            children:    vec!(),
          };

          child.children.push(new_child);
          return InsertResult::Ok;
        },
        None    => {
          if partial_key.len() > child.partial_key.len()  {
            let i = child.partial_key.len();
            return child.insert_recursive(&partial_key[i..], key, value);
          } else if partial_key.len() == child.partial_key.len() {
            if child.key_value.is_some() {
              return InsertResult::Existing;
            } else {
              child.key_value = Some((key.clone(), value));
              return InsertResult::Ok;
            }
          } else {
            // the partial key is smaller, insert as parent
            child.split(partial_key.len());
            child.key_value = Some((key.clone(), value));
            return InsertResult::Ok;
          }
        }
      };
    }

    let new_child = TrieNode {
      partial_key: partial_key.to_vec(),
      key_value:   Some((key.clone(), value)),
      children:    vec!(),
    };

    self.children.push(new_child);
    InsertResult::Ok
  }

  pub fn remove(&mut self, key: &Key) -> RemoveResult {
    self.remove_recursive(key)
  }

  pub fn remove_recursive(&mut self, partial_key: &[u8]) -> RemoveResult {
    assert_ne!(partial_key, &b""[..]);
    let mut found_child:Option<usize> = None;

    // checking directly the children
    for (index, child) in self.children.iter_mut().enumerate() {

      let pos = partial_key.iter().zip(child.partial_key.iter()).position(|(&a,&b)| a != b);
      match pos {
        Some(_) => continue,
        None    => {
          if partial_key.len() > child.partial_key.len()  {
            let i = child.partial_key.len();
            return child.remove_recursive(&partial_key[i..]);
          } else if partial_key.len() == child.partial_key.len() {
            found_child = Some(index);
            break;
          } else {
            continue
          }
        }
      };
    }

    if let Some(index) = found_child {
      if self.children[index].children.len() > 0 && self.children[index].key_value.is_some() {
        self.children[index].key_value = None;
        return RemoveResult::Ok;
      } else {
        self.children.remove(index);
        if self.key_value.is_some() {
          return RemoveResult::Ok;
        } else {
          //merging with the child
          if self.children.len() == 1 {
            let mut ch     = self.children.remove(0);
            self.key_value = ch.key_value.take();
            self.children  = ch.children;
            self.partial_key.extend(ch.partial_key);
          }
          // not handling the case of empty children vec
          // this case should only happen if it is the root node
          // otherwise, when we get to one last node and there is
          // no key_value, it is merged with the child node
          return RemoveResult::Ok
        }
      }
    } else {
      RemoveResult::NotFound
    }
  }

  pub fn lookup(&self, partial_key: &[u8]) -> Option<&KeyValue<Key,V>> {
    assert_ne!(partial_key, &b""[..]);

    if partial_key.starts_with(&self.partial_key) {
      if partial_key.len() == self.partial_key.len() {
        return self.key_value.as_ref();
      } else {
        for child in self.children.iter() {
          let res = child.lookup(&partial_key[self.partial_key.len()..]);
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

  // specific version that will handle wildcard domains
  pub fn domain_insert(&mut self, key: Key, value: V) -> InsertResult {
    let mut partial_key = key.clone();
    partial_key.reverse();
    self.insert_recursive(&partial_key, &key, value)
  }

  // specific version that will handle wildcard domains
  pub fn domain_remove(&mut self, key: &Key) -> RemoveResult {
    let mut partial_key = key.clone();
    partial_key.reverse();
    self.remove_recursive(&partial_key)
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
    let pos = partial_key.iter().zip(self.partial_key.iter()).position(|(&a,&b)| a != b);
    //println!("lookup at level: {}, testing {}", str::from_utf8(&self.partial_key).unwrap(),
    //  str::from_utf8(partial_key).unwrap());

    match pos {
      Some(i) => {
        // check for wildcard
        if i+1 == self.partial_key.len() && self.partial_key[i] == '*' as u8 {
          let c = '.' as u8;
          if (&partial_key[i..]).contains(&c) {
            None
          } else {
            self.key_value.as_ref()
          }
        } else {
          None
        }
      },
      None    => {
        if partial_key.len() > self.partial_key.len() {
          for child in self.children.iter() {
            let res = child.domain_lookup_recursive(&partial_key[self.partial_key.len()..]);
            if res.is_some() {
              return res
            }
          }
          None
        } else if partial_key.len() == self.partial_key.len() {
          self.key_value.as_ref()
        } else {
          None
        }

      }
    }
  }

  pub fn print(&self) {
    self.print_recursive(0)
  }

  pub fn print_recursive(&self, indent:u8) {
    let raw_prefix:Vec<u8> = iter::repeat(' ' as u8).take(2*indent as usize).collect();
    let prefix = str::from_utf8(&raw_prefix).unwrap();

    if let Some((ref key, ref value)) = self.key_value {
    println!("{}{}: ({},{:?})", prefix, str::from_utf8(&self.partial_key).unwrap(),
      str::from_utf8(&key).unwrap(), value);
    } else {
    println!("{}{}: None", prefix, str::from_utf8(&self.partial_key).unwrap());
    }
    for child in self.children.iter() {
      child.print_recursive(indent+1);
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
