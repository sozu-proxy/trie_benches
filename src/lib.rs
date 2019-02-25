extern crate uuid;
extern crate rand;

#[macro_use]
pub mod seed;
pub mod gen_seed;
pub mod sozu_trie;
pub mod experiment1_trie;
pub mod experiment2_trie;
pub mod experiment3_trie;
pub mod linear;
pub mod hashmap;

pub type Key = Vec<u8>;
pub type KeyValue<K,V> = (K,V);

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

pub trait DomainLookup<V> {
  fn domain_insert(&mut self, key: Key, value: V) -> InsertResult;

  // specific version that will handle wildcard domains
  fn domain_remove(&mut self, key: &Key) -> RemoveResult;

  // specific version that will handle wildcard domains
  fn domain_lookup(&self, key: &[u8]) -> Option<&KeyValue<Key,V>>;
}
