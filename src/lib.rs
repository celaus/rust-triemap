#![feature(result_map_or_else, box_into_raw_non_null, box_syntax)]

use std::boxed::Box;
use std::rc::Rc;

type Link = usize;

struct Node<K, V>
where
    K: Eq + Ord + Clone + Sized,
    V: Clone,
{
    parent: Option<Link>,
    links: Vec<Link>,
    keys: Vec<K>,
    value: Option<Rc<V>>,
}

impl<K, V> Node<K, V>
where
    K: Eq + Ord + Clone + Sized,
    V: Clone,
{
    fn new(parent: Option<Link>, value: Option<Rc<V>>) -> Node<K, V> {
        Node {
            links: vec![],
            keys: vec![],
            value: value,
            parent: parent,
        }
    }
}

pub struct TrieMap<K, V>
where
    K: Eq + Ord + Clone + Sized,
    V: Clone,
{
    keys: Vec<K>,
    links: Vec<Link>,
    data: Vec<Node<K, V>>,
    length: usize,
}

impl<K, V> TrieMap<K, V>
where
    K: Eq + Ord + Clone + Sized,
    V: Clone,
{
    pub fn new_empty() -> TrieMap<K, V> {
        TrieMap {
            keys: vec![],
            links: vec![],
            data: vec![],
            length: 0,
        }
    }

    fn len(&self) -> usize {
        self.length
    }

    pub fn insert(&mut self, key: &[K], value: V) {
        let mut key = key.iter();
        if let Some(start) = key.next() {
            let mut n = match self.keys.binary_search(&start) {
                Ok(i) => self.links[i],
                Err(i) => {
                    self.keys.insert(i, start.clone());
                    let new = Node::new(None, None);
                    self.data.push(new);
                    self.links.insert(i, self.data.len() - 1);
                    self.data.len() - 1
                }
            };

            while let Some(k) = key.next() {
                n = match self.data[n].keys.binary_search(&k) {
                    Ok(i) => self.data[n].links[i],
                    Err(i) => {
                        self.data[n].keys.insert(i, k.clone());
                        let new = Node::new(Some(n), None);
                        let new_position = self.data.len();
                        self.data[n].links.insert(i, new_position);
                        self.data.push(new);
                        new_position
                    }
                }
            }
            if self.data[n].value.is_none() {
                self.length += 1;
            }
            self.data[n].value = Some(Rc::new(value));
        }
    }

    pub fn get(&self, key: &[K]) -> Option<Rc<V>> {
        let mut key = key.iter();

        if let Some(start) = key.next() {
            match self.keys.binary_search(&start) {
                Ok(i) => {
                    let mut n = self.links[i];
                    let mut key_found = false;
                    while let Some(k) = key.next() {
                        key_found = true;
                        n = match self.data[n].keys.binary_search(&k) {
                            Ok(i) => self.data[n].links[i],
                            _ => {
                                key_found = false;
                                break;
                            }
                        }
                    }

                    if key_found {
                        self.data[n].value.clone()
                    } else {
                        None
                    }
                }
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn append(&mut self, other: &mut TrieMap<K, V>) {
        // tbd
    }
    pub fn remove(&mut self, key: &[K]) -> Option<V> {
        None
    }

    // pub fn walk(&self, callback: impl Fn(&V) -> ()) {
    //     for r in  self.root.values() {
    //         self.walk_r(&r, &callback);
    //     }
    // }

    // fn walk_r(&self, node: &Link<K, V>, callback: &impl Fn(&V) -> ()) {
    //     for n in  node.next.values() {
    //         self.walk_r(&n, callback);
    //     }
    //     if let Some(ref dev) = node.value {
    //         callback(dev);
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trie_insert() {
        let mut trie = TrieMap::<u8, usize>::new_empty();
        let len = 10;

        for i in 0..len {
            let s = format!("{:08}", i).into_bytes();
            trie.insert(&s, i);
        }

        assert_eq!(trie.len(), len);
    }
 

    #[test]
    fn trie_get() {
        let mut trie = TrieMap::<u8, usize>::new_empty();
        let len = 10;

        let mut values = vec![];
        for i in 0..len {
            let s = format!("{:08}", i).into_bytes();
            trie.insert(&s, i);
            values.push((s, i));
        }

        assert_eq!(trie.len(), len);
        assert_eq!(trie.get("100".as_bytes()), None);
        for v in values {
            assert_eq!(trie.get(&v.0), Some(Rc::new(v.1)));
        }
    }

}
