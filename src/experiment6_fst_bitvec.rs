//! try with the fst crate
//!
//! this example uses a state machine that must be completely regenerated
//! on each change, and keys must be inserted in order

use std::fmt::{Debug,Display};
use fst::{MapBuilder,Map};
use bitvec::*;

use super::{Key, KeyValue, InsertResult, RemoveResult, DomainLookup};

pub fn ascii_to_6(c: u8) -> u8 {
  //println!("will encode {} = {}", c, c as char);
  match c {
    b'A'..=b'Z' => c - 0x41,
    b'a'..=b'z' => c - 0x61,
    b'0'..=b'9' => c - 0x30 + 26,
    b'-'        => 37,
    b'.'        => 38,
    b'_'        => 39,
    b'*'        => 40,
    i           => panic!("could not convert character {} = {}", i, i as char)
  }
}

pub fn encode_6_bits(data: &[u8]) -> Vec<u8> {
  let mut vec: BitVec<BigEndian> = BitVec::new();
  for c in data.iter() {
    let c2 = ascii_to_6(*c);
    //println!("converting {} from {:b} to {:b}", c, c, c2);
    let sl0: &[u8] = &[c2];
    let sl: &BitSlice = sl0.into();
    //println!("bitslice is {}", sl);
    vec.extend(sl.iter().skip(2).take(6));
    //println!("bitvec is now {}", vec);
  }
  let v = vec.into();
  //println!("result: {:x?}", v);
  v
}

pub struct Machine<V> {
  index: Vec<KeyValue<Key,V>>,
  map: MachineMap<V>,
}

pub enum MachineMap<V> {
  Building(Vec<(Key, V)>),
  Map(Map),
}

impl<V: Ord+Display> Machine<V> {
  pub fn new() -> Self {
    Machine {
      index: Vec::new(),
      map: MachineMap::Building(Vec::new()),
    }
  }

  pub fn finish(&mut self) {
    let mut builder = MapBuilder::memory();

    match self.map {
      MachineMap::Map(_) => panic!("already finished"),
      MachineMap::Building(ref mut v) => {
        v.sort_by(|a, b| a.0.iter().rev().cmp(b.0.iter().rev()));

        let mut index = 0u64;
        for (k, v) in v.drain(..) {
          let mut key = k.to_vec();
          key.reverse();
          let key = encode_6_bits(&key);

          if let Err(e) = builder.insert(&key, index) {
            //println!("error inserting key: {:?}", e);
          } else {
            //println!("inserted {} -> ({}, {})", std::str::from_utf8(&k).unwrap(),
            //  std::str::from_utf8(&key).unwrap(), v);
            self.index.push((k, v));
            index += 1;
          }
        }
      }
    }

    //println!("{} bytes written to stream", builder.bytes_written());
    let v = builder.into_inner().unwrap();

    self.map = MachineMap::Map(Map::from_bytes(v).unwrap());
  }

  pub fn lookup(&self, key: &[u8]) -> Option<u64> {
    let mut partial_key = key.to_vec();
    partial_key.reverse();
    let partial_key = encode_6_bits(&partial_key);

    match self.map {
      MachineMap::Map(ref m) => {
        m.get(&partial_key)
      },
      MachineMap::Building(_) => {
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
      MachineMap::Map(_) => panic!("already finished"),
      MachineMap::Building(ref mut v) => {
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
    let partial_key = encode_6_bits(&partial_key);

    //println!("looking up {} -> {}", std::str::from_utf8(key).unwrap(), std::str::from_utf8(&partial_key).unwrap());

    match self.map {
      MachineMap::Map(ref m) => {
        m.get(partial_key).and_then(|i| self.index.get(i as usize))
      },
      MachineMap::Building(_) => {
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

    assert_eq!(root.domain_insert(Vec::from(&b"www.example.com"[..]), 1), InsertResult::Ok);
    assert_eq!(root.domain_insert(Vec::from(&b"test.example.com"[..]), 2), InsertResult::Ok);
    assert_eq!(root.domain_insert(Vec::from(&b"*.alldomains.org"[..]), 3), InsertResult::Ok);
    assert_eq!(root.domain_insert(Vec::from(&b"alldomains.org"[..]), 4), InsertResult::Ok);
    assert_eq!(root.domain_insert(Vec::from(&b"hello.com"[..]), 5), InsertResult::Ok);

    root.finish();

    assert_eq!(root.domain_lookup(&b"example.com"[..]), None);
    assert_eq!(root.domain_lookup(&b"blah.test.example.com"[..]), None);

    assert_eq!(root.domain_lookup(&b"www.example.com"[..]), Some(&((&b"www.example.com"[..]).to_vec(), 1)));
    assert_eq!(root.domain_lookup(&b"alldomains.org"[..]), Some(&((&b"alldomains.org"[..]).to_vec(), 4)));
    //assert_eq!(root.domain_lookup(&b"test.alldomains.org"[..]), Some(&((&b"*.alldomains.org"[..]).to_vec(), 3)));
    //assert_eq!(root.domain_lookup(&b"hello.alldomains.org"[..]), Some(&((&b"*.alldomains.org"[..]).to_vec(), 3)));
    assert_eq!(root.domain_lookup(&b"blah.test.alldomains.org"[..]), None);

  }
}
