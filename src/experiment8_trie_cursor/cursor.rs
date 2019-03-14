use std::{fmt, str::from_utf8};
use regex::bytes::Regex;

#[derive(Clone)]
pub enum Position<'a> {
  HostUri(HostIterator<'a>, &'a[u8]),
  Uri(&'a [u8]),
}

impl<'a> fmt::Display for Position<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Position::HostUri(it, uri) =>  write!(f, "{} || {}", it, from_utf8(uri).unwrap()),
      Position::Uri(uri) =>  write!(f, "_ || {}", from_utf8(uri).unwrap()),
    }
  }
}

#[derive(Clone)]
pub struct HttpCursor<'a> {
  pub position: Option<Position<'a>>,
}

impl<'a> fmt::Display for HttpCursor<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "HttpCursor {{ {} }}", self.position.as_ref().unwrap())
  }
}

impl<'a> HttpCursor<'a> {
  pub fn new(host: &'a[u8], uri: &'a[u8]) -> Self {
    HttpCursor {
      position: Some(Position::HostUri(HostIterator::new(host), uri))
    }
  }

  pub fn at_end(&self) -> bool {
    match self.position {
      Some(Position::HostUri(ref h, _)) => h.at_end(),
      Some(Position::Uri(_)) => true,
      None => panic!()
    }
  }

  pub fn current_slice(&self) -> &[u8] {
    match self.position {
      Some(Position::HostUri(ref h, _)) => h.host,
      Some(Position::Uri(ref u)) => u,
      None => panic!()
    }

  }

  pub fn match_pattern(&mut self, pattern: &MatchPattern) -> bool {
    let pos = self.position.take().unwrap();
    match pos {
      Position::HostUri(mut host, uri) => {
        match pattern {
          MatchPattern::HostPrefix(ref prefix) => {
            if host.match_prefix(prefix).is_none() && host.len() >= prefix.len() {
              host.advance(prefix.len());
              self.position = Some(Position::HostUri(host, uri));
              true
            } else {
              self.position = Some(Position::HostUri(host, uri));
              false
            }
          }
          MatchPattern::HostSniWildcard => {
            if host.match_sni_wildcard() {
              self.position = Some(Position::Uri(uri));
              true
            } else {
              self.position = Some(Position::HostUri(host, uri));
              false
            }
          },
          MatchPattern::HostRegex(ref r) => {
            match host.match_regex(r) {
              Some(sz) => {
                host.advance(sz);
                self.position = Some(Position::HostUri(host, uri));
                true
              },
              None => {
                self.position = Some(Position::HostUri(host, uri));
                false
              }
            }
          }
          MatchPattern::UriPrefix(ref prefix) => {
            if !host.at_end() {
              self.position = Some(Position::HostUri(host, uri));
              false
            } else {
              match uri.iter().zip(prefix.iter()).position(|(&a,&b)| { a != b }) {
                Some(pos) => {
                  self.position = Some(Position::Uri(uri));
                  false
                },
                None => {
                  if prefix.len() <= uri.len() {
                    self.position = Some(Position::Uri(&uri[prefix.len()..]));
                    true
                  } else {
                    self.position = Some(Position::Uri(uri));
                    false
                  }
                }
              }
            }
          }
          MatchPattern::UriRegex(ref r) => {
            if !host.at_end() {
              self.position = Some(Position::HostUri(host, uri));
              false
            } else {
              if r.is_match(uri) {
                self.position = Some(Position::Uri(uri));
                true
              } else {
                self.position = Some(Position::Uri(uri));
                false
              }
            }
          }
        }
      }
      Position::Uri(uri) => {
        match pattern {
          MatchPattern::UriPrefix(ref prefix) => {
            match uri.iter().zip(prefix.iter()).position(|(&a,&b)| { a != b }) {
              Some(pos) => {
                self.position = Some(Position::Uri(uri));
                false
              },
              None => {
                if prefix.len() <= uri.len() {
                  self.position = Some(Position::Uri(&uri[prefix.len()..]));
                  true
                } else {
                  self.position = Some(Position::Uri(uri));
                  false
                }
              }
            }
          }
          MatchPattern::UriRegex(ref r) => {
            if r.is_match(uri) {
              self.position = Some(Position::Uri(uri));
              true
            } else {
              self.position = Some(Position::Uri(uri));
              false
            }
          }
          _ => {
            self.position = Some(Position::Uri(uri));
            false
          }
        }
      }
    }
  }

  pub fn match_host_prefix(&mut self, prefix: &[u8]) -> bool {
    let pos = self.position.take().unwrap();
    match pos {
      Position::HostUri(mut host, uri) => {
        if host.match_prefix(prefix).is_none() && host.len() >= prefix.len() {
          host.advance(prefix.len());
          self.position = Some(Position::HostUri(host, uri));
          true
        } else {
          self.position = Some(Position::HostUri(host, uri));
          false
        }
      }
      position => {
        self.position = Some(position);
        false
      }
    }
  }

  pub fn match_host_wildcard(&mut self) -> bool {
    let pos = self.position.take().unwrap();
    match pos {
      Position::HostUri(mut host, uri) => {
        if host.match_sni_wildcard() {
          self.position = Some(Position::Uri(uri));
          true
        } else {
          self.position = Some(Position::HostUri(host, uri));
          false
        }
      }
      position => {
        self.position = Some(position);
        false
      }
    }
  }

  pub fn match_host_regex(&mut self, r: &Regex) -> bool {
    let pos = self.position.take().unwrap();
    match pos {
      Position::HostUri(mut host, uri) => {
        match host.match_regex(r) {
          Some(sz) => {
            host.advance(sz);
            self.position = Some(Position::HostUri(host, uri));
            true
          },
          None => {
            self.position = Some(Position::HostUri(host, uri));
            false
          }
        }
      }
      position => {
        self.position = Some(position);
        false
      }
    }
  }

  pub fn match_uri_prefix(&mut self, prefix: &[u8]) -> bool {
    let pos = self.position.take().unwrap();
    match pos {
      Position::HostUri(mut host, uri) => {
        if !host.at_end() {
          self.position = Some(Position::HostUri(host, uri));
          false
        } else {
          match uri.iter().zip(prefix.iter()).position(|(&a,&b)| { a != b }) {
            Some(pos) => {
              self.position = Some(Position::Uri(uri));
              false
            },
            None => {
              if prefix.len() <= uri.len() {
                self.position = Some(Position::Uri(&uri[prefix.len()..]));
                true
              } else {
                self.position = Some(Position::Uri(uri));
                false
              }
            }
          }
        }
      }
      Position::Uri(uri) => {
        match uri.iter().zip(prefix.iter()).position(|(&a,&b)| { a != b }) {
          Some(pos) => {
            self.position = Some(Position::Uri(uri));
            false
          },
          None => {
            if prefix.len() <= uri.len() {
              self.position = Some(Position::Uri(&uri[prefix.len()..]));
              true
            } else {
              self.position = Some(Position::Uri(uri));
              false
            }
          }
        }
      }
    }
  }

  pub fn match_uri_regex(&mut self, r: &Regex) -> bool {
    let pos = self.position.take().unwrap();
    match pos {
      Position::HostUri(mut host, uri) => {
        if !host.at_end() {
          self.position = Some(Position::HostUri(host, uri));
          false
        } else {
          if r.is_match(uri) {
            self.position = Some(Position::Uri(uri));
            true
          } else {
            self.position = Some(Position::Uri(uri));
            false
          }
        }
      }
      Position::Uri(uri) => {
        if r.is_match(uri) {
          self.position = Some(Position::Uri(uri));
          true
        } else {
          self.position = Some(Position::Uri(uri));
          false
        }
      }
    }
  }

  pub fn advance(&mut self, mut offset: usize) -> bool {
    let pos = self.position.take().unwrap();
    match pos {
      Position::HostUri(mut host, uri) => {
        let host_len = host.len();
        if offset > host_len {
          host.advance(host_len);
          offset -= host_len;

          if offset > uri.len() {
            return false;
          } else {
            self.position = Some(Position::Uri(&uri[offset..]));
          }
        } else if offset == host_len {
          host.advance(offset);
          self.position = Some(Position::Uri(uri));
        } else {
          host.advance(offset);
          self.position = Some(Position::HostUri(host, uri));
        }
      }
      Position::Uri(uri) => {
        if offset > uri.len() {
          return false;
        } else {
          self.position = Some(Position::Uri(&uri[offset..]));
        }
      }
    }

    true
  }


  pub fn next_pattern(&self) -> Option<(usize, MatchPattern)> {
    match self.position.as_ref() {
      None => None,
      Some(Position::HostUri(host, uri)) => {
        host.next_pattern()
      },
      Some(Position::Uri(uri)) => {
        if uri.is_empty() {
          return None;
        }

        if uri[0] == b'~' {
          Some((uri.len() - 1, MatchPattern::UriRegex(Regex::new(from_utf8(&uri[1..]).unwrap()).unwrap())))
        } else {
          Some((uri.len(),  MatchPattern::UriPrefix(uri.to_vec())))
        }
      },
    }
  }
}

#[derive(Debug,Clone)]
pub enum MatchPattern {
  HostPrefix(Vec<u8>),
  HostSniWildcard,
  HostRegex(Regex),
  UriPrefix(Vec<u8>),
  UriRegex(Regex),
}

impl fmt::Display for MatchPattern {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      MatchPattern::HostPrefix(v) =>  write!(f, "HostPrefix({})", from_utf8(v).unwrap()),
      MatchPattern::HostSniWildcard =>  write!(f, "HostSniWildcard"),
      MatchPattern::HostRegex(r) =>  write!(f, "HostRegex({})", r.as_str()),
      MatchPattern::UriPrefix(v) =>  write!(f, "UriPrefix({})", from_utf8(v).unwrap()),
      MatchPattern::UriRegex(r) =>  write!(f, "UriRegex({})", r.as_str()),
    }
  }
}

impl PartialEq for MatchPattern {
  fn eq(&self, other: &MatchPattern) -> bool {
    match (self, other) {
      (&MatchPattern::HostRegex(ref r1),  &MatchPattern::HostRegex(ref r2)) => r1.as_str() == r2.as_str(),
      (&MatchPattern::UriRegex(ref r1),   &MatchPattern::UriRegex(ref r2)) => r1.as_str() == r2.as_str(),
      (&MatchPattern::HostPrefix(ref p1), &MatchPattern::HostPrefix(ref p2)) => p1 == p2,
      (&MatchPattern::UriPrefix(ref p1),  &MatchPattern::UriPrefix(ref p2)) => p1 == p2,
      (&MatchPattern::HostSniWildcard,    &MatchPattern::HostSniWildcard) => true,
      _ => false,
    }
  }
}


fn find_last_dot(input: &[u8]) -> Option<usize> {
  println!("find_last_dot: input = {}", from_utf8(input).unwrap());
  for i in (0..input.len()).rev() {
    println!("input[{}] -> {}", i, input[i] as char);
    if input[i] == b'.' {
      return Some(i);
    }
  }

  None
}

#[derive(Debug,Clone,PartialEq)]
pub struct HostIterator<'a> {
  pub host: &'a[u8],
}

impl<'a> HostIterator<'a> {
  pub fn new(host: &'a[u8]) -> Self {
    HostIterator { host }
  }

  pub fn at_end(&self) -> bool {
    self.host.is_empty()
  }

  pub fn len(&self) -> usize {
    self.host.len()
  }

  pub fn advance(&mut self, offset: usize) {
    assert!(offset <= self.host.len(), "advancing should not go further than the hostname size");

    let len = self.host.len() - offset;
    self.host = &self.host[..len];
  }

  pub fn match_prefix(&self, prefix: &[u8]) -> Option<usize> {
    self.host.iter().rev().zip(prefix.iter().rev()).position(|(&a,&b)| {
      println!("match_prefix: testing {} ?= {}", a as char, b as char);
      a != b
    })
  }

  pub fn match_next_char(&self, keys: &[u8]) -> Option<usize> {
    if self.host.is_empty() {
      return None;
    }

    keys.iter().position(|k| *k == self.host[self.host.len() - 1])
  }

  pub fn match_sni_wildcard(&self) -> bool {
    !self.host.contains(&b'.')
  }

  pub fn match_regex(&self, r: &Regex) -> Option<usize> {
    let sl = match find_last_dot(self.host) {
      Some(pos) => &self.host[pos+1..],
      None => &self.host
    };

    println!("match regex: testing /{}/ on {}", r.as_str(), from_utf8(sl).unwrap());
    if r.is_match(sl) {
      Some(sl.len())
    } else {
      None
    }
  }

  pub fn next_pattern(&self) -> Option<(usize, MatchPattern)> {
    if self.host.is_empty() {
      return None;
    }

    if self.host[self.host.len() - 1] == b'*' {
      Some((1, MatchPattern::HostSniWildcard))
    } else if self.host[self.host.len() - 1] == b'/' {
      match find_last_dot(self.host) {
        None => if self.host[0] == b'/' {
          Some((self.host.len(), MatchPattern::HostRegex(Regex::new(from_utf8(self.host).unwrap()).unwrap())))
        } else {
          None
        },
        Some(pos) => if self.host[pos+1] == b'/' {
          Some(((&self.host[pos+1..]).len(), MatchPattern::HostRegex(Regex::new(from_utf8(&self.host[pos+1..]).unwrap()).unwrap())))
        } else {
          None
        }
      }
    } else {
      let mut host_end = self.host.len();

      loop {
        if self.host[host_end-1] == b'/' || self.host[host_end-1] == b'*' {
          if host_end == self.host.len() {
            return None;
          }
          return Some(((&self.host[host_end..]).len(), MatchPattern::HostPrefix((&self.host[host_end..]).to_vec())));
        }

        match find_last_dot(&self.host[..host_end-1]) {
          None => return Some((self.host.len(), MatchPattern::HostPrefix(self.host.to_vec()))),
          Some(pos) => host_end = pos,
        }
      }
    }
  }
}

impl<'a> fmt::Display for HostIterator<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "HostIt {{ {} }}", from_utf8(self.host).unwrap())
  }
}

pub fn make_match_patterns(host: &[u8], uri_prefix: Option<&[u8]>, uri_regex: Option<&str>) -> Vec<MatchPattern> {
  let mut v = vec![];
  let mut host_end = host.len();
  loop {
    match find_last_dot(&host[..host_end]) {
      None => {
        if host[host_end-1] == b'*' {
          v.push(MatchPattern::HostSniWildcard);
        } else {
          if host[0] == b'/' && host[host_end-1] == b'/' {
            v.push(MatchPattern::HostRegex(Regex::new(from_utf8(&host[1..host_end-1]).unwrap()).unwrap()));
          } else {
            v.push(MatchPattern::HostPrefix((&host[..host_end]).to_vec()));
          }
        }
        break;
       },
      Some(pos) => {
        if host[pos] == b'/' && host[host_end-1] == b'/' {
          v.push(MatchPattern::HostRegex(Regex::new(from_utf8(&host[pos+1..host_end-1]).unwrap()).unwrap()));
        } else {
          v.push(MatchPattern::HostPrefix((&host[pos..host_end]).to_vec()));
        }
        host_end = pos;
      }
    }
  }

  if uri_prefix.is_some() && uri_regex.is_some() {
    panic!("no uri prefix and regex at the same time");
  }

  if let Some(prefix) = uri_prefix {
    v.push(MatchPattern::UriPrefix(prefix.to_vec()));
  } else if let Some(regex) = uri_regex {
    v.push(MatchPattern::UriRegex(Regex::new(regex).unwrap()));
  }

  v
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn host_iterator() {
    let mut h1 = HostIterator::new(&b"api.example.com"[..]);
    println!("h1: {}", h1);
    h1.advance(4);
    println!("h1: {}", h1);
    h1.advance(5);
    println!("h1: {}", h1);
    h1.advance(3);
    println!("h1: {}", h1);

    assert!(!h1.at_end());

    h1.advance(3);
    println!("h1: {}", h1);

    assert!(h1.at_end());

    //panic!();
  }

  #[test]
  fn prefix() {
    let mut h1 = HostIterator::new(&b"api.example.com"[..]);
    println!("h1: {}", h1);
    let prefix = &b".com"[..];
    assert_eq!(None, h1.match_prefix(prefix));
    if h1.len() >= prefix.len() {
      h1.advance(prefix.len());
    }
    println!("h1: {}", h1);
    assert!(!h1.at_end());

    let prefix = &b"xample"[..];
    assert_eq!(None, h1.match_prefix(prefix));
    if h1.len() >= prefix.len() {
      h1.advance(prefix.len());
    }
    println!("h1: {}", h1);
    assert!(!h1.at_end());

    let prefix = &b".e"[..];
    assert_eq!(None, h1.match_prefix(prefix));
    if h1.len() >= prefix.len() {
      h1.advance(prefix.len());
    }
    println!("h1: {}", h1);
    assert!(!h1.at_end());

    assert!(h1.match_sni_wildcard());

    //panic!();
  }

  #[test]
  fn regex() {
    let mut h1 = HostIterator::new(&b"js.cdn1.example.com"[..]);
    println!("h1: {}", h1);
    let prefix = &b".example.com"[..];
    assert_eq!(None, h1.match_prefix(prefix));
    if h1.len() >= prefix.len() {
      h1.advance(prefix.len());
    }
    println!("h1: {}", h1);
    assert!(!h1.at_end());

    let r = Regex::new("cdn[0-9]+").unwrap();
    assert_eq!(Some(4), h1.match_regex(&r));

    h1.advance(4);
    println!("h1: {}", h1);

    let prefix = &b"js."[..];
    assert_eq!(None, h1.match_prefix(prefix));
    if h1.len() >= prefix.len() {
      h1.advance(prefix.len());
    }
    println!("h1: {}", h1);
    assert!(h1.at_end());

    //panic!();
  }

  #[test]
  fn patterns() {
    let c = HttpCursor::new(&b"cdn12.example.com"[..], &b"/hello/world"[..]);

    println!("starting cursor: {}", c);

    let patterns = make_match_patterns(&b"cdn12.example.com"[..], Some(&b"/"[..]), None);
    let mut c1 = c.clone();
    for pattern in patterns.iter() {
      println!("testing pattern: {}", pattern);
      assert!(c1.match_pattern(pattern));
      println!("cursor = {}", c1);
    }
    assert!(c1.at_end());

    let patterns = make_match_patterns(&b"*.example.com"[..], None, Some("^/h(ello|allo)"));
    let mut c2 = c.clone();
    for pattern in patterns.iter() {
      println!("testing pattern: {}", pattern);
      assert!(c2.match_pattern(pattern));
      println!("cursor = {}", c2);
    }
    assert!(c2.at_end());

    let patterns = make_match_patterns(&b"/cdn[a-z0-9]+/.example.com"[..], None, Some("^/h(ello|allo)"));
    let mut c3 = c.clone();
    for pattern in patterns.iter() {
      println!("testing pattern: {}", pattern);
      assert!(c3.match_pattern(pattern));
      println!("cursor = {}", c2);
    }
    assert!(c3.at_end());
    //panic!();
  }

  #[test]
  fn next_pattern() {
    let mut c = HttpCursor::new(&b"cdn12.example.com"[..], &b"/hello/world"[..]);
    let pat = c.next_pattern().unwrap();
    println!("{} next pattern: ({}, {})", c, pat.0, pat.1);
    assert_eq!(pat, (17, MatchPattern::HostPrefix(b"cdn12.example.com".to_vec())));

    c.advance(17);
    let pat = c.next_pattern().unwrap();
    println!("{} next pattern: ({}, {})", c, pat.0, pat.1);
    assert_eq!(pat, (12, MatchPattern::UriPrefix(b"/hello/world".to_vec())));

    let mut c2 = HttpCursor::new(&b"*.example.com"[..], &b"~/(abc|def)"[..]);
    let pat = c2.next_pattern().unwrap();
    println!("{} next pattern: ({}, {})", c2, pat.0, pat.1);
    assert_eq!(pat, (12, MatchPattern::HostPrefix(b".example.com".to_vec())));

    c2.advance(12);
    let pat = c2.next_pattern().unwrap();
    println!("{} next pattern: ({}, {})", c2, pat.0, pat.1);
    assert_eq!(pat, (1, MatchPattern::HostSniWildcard));

    c2.advance(1);
    let pat = c2.next_pattern().unwrap();
    println!("{} next pattern: ({}, {})", c2, pat.0, pat.1);
    assert_eq!(pat, (10, MatchPattern::UriRegex(Regex::new("/(abc|def)").unwrap())));

    panic!();
  }
}
