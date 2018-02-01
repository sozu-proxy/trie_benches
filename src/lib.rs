#![feature(macro_rules)]

extern crate uuid;
extern crate rand;

pub mod seed;
pub mod gen_seed;
pub mod sozu_trie;

use std::{iter,str};
use std::fmt::Debug;

