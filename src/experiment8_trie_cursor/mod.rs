//! this is an evolution of experiment 3
//!
//! main differences:
//! - instead of reversing the entire domain (api.example.com -> moc.elpmaxe.ipa), we reverse the label order
//! (api.example.com -> com.example.api). This will make it easier to integrate smarter pattern matching like regexps
//! - we will integrate URL prefixes or patterns to the queries. Sozu's current design stores a vector of path prefies to match to applications. This matches closely Clever Cloud's architecture (lots of applications with each their own domain, few path prefixes per domain), but not the more common case of a few domains and lots of paths. So we will define an element that can iterate on reversed domain labels, then the path, and test patterns on it
//! - trie nodes will be able to match patterns instead of prefixes, and we will be able to extract usage specific matching outside of the trie (like SNI domain matching)


pub mod cursor;
pub mod trie;

use self::trie::*;
use super::{Key, KeyValue, InsertResult, RemoveResult, DomainLookup};

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn insert() {
    let mut root: TrieNode<u8> = TrieNode::root();
    root.print();

    assert_eq!(root.domain_insert(Vec::from(&b"test.com"[..]), 1), InsertResult::Ok);
    root.print();
    assert_eq!(root.domain_insert(Vec::from(&b"www.example.com"[..]), 2), InsertResult::Ok);
    root.print();
    assert_eq!(root.domain_insert(Vec::from(&b"cdn./a[0-9]*/.example.com"[..]), 3), InsertResult::Ok);
    root.print();
    assert_eq!(root.domain_insert(Vec::from(&b"*.js.example.com"[..]), 4), InsertResult::Ok);
    root.print();

    //assert_eq!(root.lookup(&b"abce"[..]), Some(&((&b"abce"[..]).to_vec(), 2)));
    panic!();
  }

  #[test]
  fn size() {
    assert_eq!(160, ::std::mem::size_of::<TrieNode<usize>>());
  }

/*
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

  pub struct ReducedTrieNode<V> {
    key_value:  Option<KeyValue<Key,V>>,
    local_key:  Key,
    child_keys: [u8; 8],
    children:   Vec<ReducedTrieNode<V>>,
  }

  pub struct TrieNode2<V> {
    key_value:  Option<KeyValue<Key,V>>,
    local_key:  Key,
    child_keys: Vec<u8>,
    children:   Vec<TrieNode<V>>,
    regex_children: Vec<TrieNode<V>>,
  }

  pub struct RegexNode2<V> {
    key_value:  Option<KeyValue<Key,V>>,
    local_key:  regex::bytes::Regex,
    child: Option<super::super::experiment3_trie::TrieNode<V>>,
  }

  pub struct TrieNode3<V> {
    key_value:  Option<KeyValue<Key,V>>,
    local_key:  Key,
    child_keys: Vec<u8>,
    local_regex: regex::bytes::RegexSet,
    children:   Vec<TrieNode3<V>>,
    regex_children: Vec<TrieNode3<V>>,
  }

  struct TrieNode4<V> {
    key_value: Option<KeyValue<Key, V>>,
    matcher: TrieNode4Match<V>,
  }

  enum TrieNode4Match<V> {
    //None,
    HostPrefix(TrieNode4Prefix<V>),
    //Regex(TrieNode4Regex<V>),
    HostRegex(TrieNode4Regex2<V>),
    HostWildCard(TrieNode4Wildcard),
    UriPrefix(TrieNode4Prefix<V>),
    UriRegex(TrieNode4UriRegex),
  }

  pub struct TrieNode4Prefix<V> {
    local_key:  Key,
    child_keys: Vec<u8>,
    children:   Vec<TrieNode4<V>>,
  }

  pub struct TrieNode4Regex<V> {
    local_regex: regex::bytes::RegexSet,
    regex_children: Vec<TrieNode4<V>>,
  }

  pub struct TrieNode4Regex2<V> {
    local_regexes: Vec<regex::bytes::Regex>,
    regex_children: Vec<TrieNode4<V>>,
  }

  pub struct TrieNode4Wildcard;

  pub struct TrieNode4UriRegex {
    regex: regex::bytes::Regex,
  }


  #[test]
  fn size() {
    assert_eq!(104, ::std::mem::size_of::<super::super::experiment3_trie::TrieNode<usize>>());
    assert_eq!(88, ::std::mem::size_of::<ReducedTrieNode<usize>>());
    assert_eq!(128, ::std::mem::size_of::<TrieNode2<usize>>());
    assert_eq!(192, ::std::mem::size_of::<RegexNode2<usize>>());
    assert_eq!(184, ::std::mem::size_of::<TrieNode3<usize>>());
    assert_eq!(112, ::std::mem::size_of::<TrieNode4<usize>>());
    assert_eq!(80, ::std::mem::size_of::<TrieNode4Match<usize>>());
    assert_eq!(72,  ::std::mem::size_of::<TrieNode4Prefix<usize>>());
    assert_eq!(80, ::std::mem::size_of::<TrieNode4Regex<usize>>());
    assert_eq!(48, ::std::mem::size_of::<TrieNode4Regex2<usize>>());
    assert_eq!(0, ::std::mem::size_of::<TrieNode4Wildcard>());
  }
*/
}
