//! this example uses a list of ACL tested in linear order

use super::{Key, InsertResult, RemoveResult, DomainLookup};
use hashbrown::HashMap;

pub struct Map(HashMap<Vec<u8>, (Vec<u8>, u8)>);

impl Map {
  pub fn new() -> Self {
    Map(HashMap::new())
  }
}

impl DomainLookup<u8> for Map {
  fn domain_insert(&mut self, key: Vec<u8>, value: u8) -> InsertResult {
    let mut partial_key = key.clone();
    partial_key.reverse();
    self.0.insert(partial_key, (key.clone(), value));
    InsertResult::Ok
  }

  fn domain_lookup(&self, key: &[u8]) -> Option<&(Vec<u8>, u8)> {
    let mut partial_key = key.to_vec();
    partial_key.reverse();
    self.0.get(&partial_key)
  }

  fn domain_remove(&mut self, _key: &Key) -> RemoveResult {
    unimplemented!();
  }
}
