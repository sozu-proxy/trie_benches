use std::{iter,str};
use std::fmt::Debug;

use super::{Key, KeyValue, InsertResult, RemoveResult, DomainLookup};
use super::cursor::*;

#[derive(Clone,Debug)]
pub struct TrieNode<V> {
  key_value: Option<KeyValue<Key, V>>,
  prefix: Key,
  child_keys: Vec<u8>,
  regexes: Vec<regex::bytes::Regex>,
  wildcard: Option<Box<TrieNode<V>>>,
  children: Vec<TrieNode<V>>,
  regex_children: Vec<TrieNode<V>>,
}

impl<V: PartialEq> PartialEq for TrieNode<V> {
  fn eq(&self, other: &TrieNode<V>) -> bool {
    if self.key_value == other.key_value &&
      self.prefix == other.prefix &&
        self.child_keys == other.child_keys &&
        self.wildcard == other.wildcard &&
        self.children == other.children &&
        self.regex_children == other.regex_children &&
        self.regexes.len() == other.regexes.len() {
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
      prefix: vec![],
      child_keys: vec![],
      regexes: vec![],
      wildcard: None,
      children: vec![],
      regex_children: vec![],
    }
  }

  //pub fn insert<'a>(&mut self, key: Key, value: V) -> InsertResult {
  pub fn insert<'a>(&mut self, mut cursor: HttpCursor<'a>, value: V) -> InsertResult {
    //println!("insert: testing {}", cursor);
    if cursor.at_end() {
      self.key_value = Some((self.prefix.clone(), value));
      return InsertResult::Ok;
    }

    let res = match cursor.match_prefix_position(&self.prefix) {
      None => match cursor.next_pattern_type() {
        MatchPatternType::Regex => {
          if let Some((sz, MatchPattern::Regex(r))) = cursor.next_pattern() {
            cursor.advance(sz);
            match self.regexes.iter().position(|reg| reg.as_str() == r.as_str()) {
              Some(c) => {
                self.regex_children[c].insert(cursor, value)
              }
              None => {
                let mut node = TrieNode::root();
                match node.insert(cursor, value) {
                  InsertResult::Ok => {
                    self.regexes.push(r);
                    self.regex_children.push(node);
                    InsertResult::Ok
                  },
                  res => res
                }
              }
            }
          } else {
            panic!("next pattern should be a regex");
            InsertResult::Failed
          }
        }
        MatchPatternType::SniWildcard => {
          cursor.advance(1);
          if self.wildcard.is_none() {
            let mut node = TrieNode::root();
            match node.insert(cursor, value) {
              InsertResult::Ok => {
                //self.wildcard = Some(Box::new(node));
                InsertResult::Ok
              },
              res => res
            }
          } else {
            let node = self.wildcard.as_mut().unwrap();
            node.insert(cursor, value)
          }
        }
        MatchPatternType::Prefix(c) => {
          //println!("prefix match found no difference with prefix \"{}\"", str::from_utf8(&self.prefix).unwrap());

          if !self.prefix.is_empty() {
            cursor.advance(1);
            match self.child_keys.iter().position(|k| *k == c) {
              Some(index) => {
                self.children[index].insert(cursor, value)
              },
              None => {
                let mut node = TrieNode::root();
                //println!("inserting new node with cursor {}", cursor);
                match node.insert(cursor, value) {
                  InsertResult::Ok => {
                    self.child_keys.push(c);
                    self.children.push(node);
                    InsertResult::Ok
                  },
                  res => res
                }
              }
            }
          } else {
            //println!("self.prefix is empty, cursor is {}", cursor);

            let mut new_node = TrieNode::root();
            match cursor.next_pattern() {
              Some((sz, MatchPattern::Prefix(prefix))) => {
                if self.child_keys.is_empty() && self.key_value.is_none() {
                  cursor.advance(sz);
                  self.prefix = prefix;
                }

                if !cursor.at_end() {
                  match cursor.next_pattern_type() {
                    MatchPatternType::Prefix(c) => {
                      cursor.advance(1);

                      match self.child_keys.iter().position(|k| *k == c) {
                        Some(index) => {
                          self.children[index].insert(cursor, value)
                        },
                        None => {
                          let mut node = TrieNode::root();
                          //println!("inserting new node with cursor {}", cursor);
                          match node.insert(cursor, value) {
                            InsertResult::Ok => {
                              self.child_keys.push(c);
                              self.children.push(node);
                              InsertResult::Ok
                            },
                            res => res
                          }
                        }
                      }
                    }
                    MatchPatternType::Regex => {
                      if let Some((sz, MatchPattern::Regex(r))) = cursor.next_pattern() {
                        cursor.advance(sz);
                        match new_node.insert(cursor, value) {
                          InsertResult::Ok => {
                            self.regexes.push(r);
                            self.regex_children.push(new_node);
                            InsertResult::Ok
                          },
                          res => res
                        }
                      } else {
                        panic!("next pattern should be a regex");
                        InsertResult::Failed
                      }
                    }
                    MatchPatternType::SniWildcard => {
                      cursor.advance(1);
                      match new_node.insert(cursor, value) {
                        InsertResult::Ok => {
                          self.wildcard = Some(Box::new(new_node));
                          InsertResult::Ok
                        },
                        res => res
                      }
                    }
                  }
                } else {
                  //println!("cursor is at end: {}", cursor);
                  self.key_value = Some((self.prefix.clone(), value));
                  InsertResult::Ok
                }
              }
              Some((sz, MatchPattern::Regex(r))) => {
                cursor.advance(sz);
                match new_node.insert(cursor, value) {
                  InsertResult::Ok => {
                    self.regexes.push(r);
                    self.regex_children.push(new_node);
                    InsertResult::Ok
                  },
                  res => res
                }
              }
              Some((sz, MatchPattern::SniWildcard)) => {
                cursor.advance(1);
                match new_node.insert(cursor, value) {
                  InsertResult::Ok => {
                    self.wildcard = Some(Box::new(new_node));
                    InsertResult::Ok
                  },
                  res => res
                }
              },
              None => InsertResult::Failed
            }

          }
        }
      }
      Some(index) => {
        println!("prefix match difference at {}", index);
        let mut node = TrieNode::root();
        let c = self.prefix[self.prefix.len() - index - 1];

        let len = self.prefix.len();
        let v = self.prefix.split_off(len - index);

        println!("splitting prefix between {} and {}", std::str::from_utf8(&self.prefix).unwrap(),
          std::str::from_utf8(&v).unwrap());
        node.prefix = std::mem::replace(&mut self.prefix, v);
        if node.prefix.len() > 0 {
          let len = node.prefix.len() - 1;
          node.prefix.truncate(len);
        }

        //println!("child node prefix: {}", std::str::from_utf8(self.pref
        //node.prefix = (&self.prefix[..index]).to_vec();
        node.regexes.extend(self.regexes.drain(..));
        node.child_keys.extend(self.child_keys.drain(..));
        node.wildcard = self.wildcard.take();
        node.key_value = self.key_value.take();
        node.children.extend(self.children.drain(..));
        node.regex_children.extend(self.regex_children.drain(..));
        println!("creating new child node from current:");
        node.print();

        self.child_keys.push(c);
        self.children.push(node);

        let mut new_node = TrieNode::root();
        match cursor.next_pattern_type() {
          MatchPatternType::Prefix(c) => {
            cursor.advance(1);
            match new_node.insert(cursor, value) {
              InsertResult::Ok => {
                self.child_keys.push(c);
                self.children.push(new_node);
                InsertResult::Ok
              },
              res => res
            }
          }
          MatchPatternType::Regex => {
            if let Some((sz, MatchPattern::Regex(r))) = cursor.next_pattern() {
              cursor.advance(sz);
              match new_node.insert(cursor, value) {
                InsertResult::Ok => {
                  self.regexes.push(r);
                  self.regex_children.push(new_node);
                  InsertResult::Ok
                },
                res => res
              }
            } else {
              panic!("next pattern should be a regex");
              InsertResult::Failed
            }
          }
          MatchPatternType::SniWildcard => {
            cursor.advance(1);
            match new_node.insert(cursor, value) {
              InsertResult::Ok => {
                self.wildcard = Some(Box::new(new_node));
                InsertResult::Ok
              },
              res => res
            }
          }
        }
      }
    };

    println!("returned {:?}", res);
    res
  }

  pub fn lookup(&self, mut cursor: HttpCursor) -> Option<&KeyValue<Key,V>> {
    //println!("looking up {}", cursor);

    match cursor.match_prefix_position(&self.prefix) {
      Some(pos) => {
        //println!("prefix difference at {}", pos);
        return None;
      }
      None => {
        if cursor.at_end() {
          return self.key_value.as_ref();
        }

        let c = cursor.next_char();
        if let Some(index) = self.child_keys.iter().position(|k| *k == c) {
          let mut cursor2 = cursor.clone();
          cursor2.advance(1);
          if let Some(kv) = self.children[index].lookup(cursor2) {
            return Some(kv);
          }
        }

        for (i, r) in self.regexes.iter().enumerate() {
          let mut cursor2 = cursor.clone();
          if cursor2.match_regex(r) {
            if let Some(kv) = self.regex_children[i].lookup(cursor2) {
              return Some(kv);
            }
          }
        }

        if let Some(child) = self.wildcard.as_ref() {
          if cursor.match_sni_wildcard() {
            if let Some(kv) = child.lookup(cursor) {
              return Some(kv);
            }
          }
        }

        return self.key_value.as_ref();
      },
    }
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


  pub fn print(&self) {
    self.print_recursive(b'.', 0)
  }

  pub fn print_recursive(&self, partial_key: u8, indent:u8) {
    let raw_prefix:Vec<u8> = iter::repeat(' ' as u8).take(2*indent as usize).collect();
    let prefix = str::from_utf8(&raw_prefix).unwrap();
    let c: char = partial_key.into();

    if let Some((ref key, ref value)) = self.key_value {
      println!("{}{}: {}|{:?}", prefix, c,
        str::from_utf8(&key).unwrap(), value);
      //println!("{}{}: {}|({},{:?})", prefix, c,
      //  str::from_utf8(&self.prefix).unwrap(),
      //  str::from_utf8(&key).unwrap(), value);
    } else {
      println!("{}{}: {}", prefix, c,
        str::from_utf8(&self.prefix).unwrap());
    }
    for (child_key, ref child) in self.child_keys.iter().zip(self.children.iter()) {
      child.print_recursive(*child_key, indent+1);
    }
    for (ref regex, ref child) in self.regexes.iter().zip(self.regex_children.iter()) {
      println!("  {}/{}/", prefix, regex.as_str());
      child.print_recursive(b'~', indent+2);
    }
    if let Some(child) = self.wildcard.as_ref() {
      child.print_recursive(b'*', indent+1);
    }
  }
}

impl<V: Debug> DomainLookup<V> for TrieNode<V> {
  fn domain_insert(&mut self, key: Key, value: V) -> InsertResult {
    let cursor = HttpCursor::new(&key, &b"/"[..]);
    self.insert(cursor, value)
  }

  // specific version that will handle wildcard domains
  fn domain_remove(&mut self, key: &Key) -> RemoveResult {
    //unimplemented!();
    panic!()
    /*
    let mut partial_key = key.clone();
    partial_key.reverse();
    self.remove(&partial_key)
    */
  }

  // specific version that will handle wildcard domains
  fn domain_lookup(&self, key: &[u8]) -> Option<&KeyValue<Key,V>> {
    let cursor = HttpCursor::new(&key, &b"/"[..]);
    self.lookup(cursor)
  }
}
