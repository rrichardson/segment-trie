#![feature(stdsimd)]

use std::iter::Iterator;
use std::hash::{Hash, Hasher};
use fasthash::city::CityHasher64;

const BRANCHING_FACTOR: usize = 32;
type KeyHash=u32;

pub struct TrieNode<K, V> {
    keychain: Vec<KeyHash>, 
    key: K,
    values: Vec<V>,
    childkeys: u32,
    children: Vec<Option<Box<TrieNode<K,V>>>>,
}

pub struct Trie<K, V> {
    root: Option<TrieNode<K, V>>,
}

impl<K, V>  Trie<K, V>
where K: Hash
{
    pub fn new() -> Trie<K, V> {
        Trie {
            root: None,
        }
    }
}

impl<K, V>  TrieNode<K, V>
where K: Hash + PartialEq
{
    pub fn new(key: K, val: V) -> TrieNode<K, V> {
        TrieNode {
            keychain: vec![Self::hashkey(&key)],
            key,
            values: vec![val],
            childkeys: 0,
            children: vec![],
        }
    }

    pub fn insert<I: Iterator<Item=K>>(mut key_segments: I, val: V) -> bool {
        let keyhashes = key_segments.map(|k| Self::hashkey(&k)).collect::<Vec<KeyHash>>();

        true
    }

    fn hashkey(key: &K) -> u32 {
        let mut hasher : CityHasher64 = Default::default();
        key.hash(&mut hasher);
        hasher.finish() as u32
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
