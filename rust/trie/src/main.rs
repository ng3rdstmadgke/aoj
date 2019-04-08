fn main() {
    println!("Hello, world!");
}


#[derive(Debug)]
struct Trie<K: Ord + Eq + Copy, V> {
    root: Node<K, V>
}

#[derive(Debug)]
struct Node<K: Ord + Eq + Copy, V> {
    key  : Option<K>,
    value: Option<V>,
    next : Vec<Node<K, V>>,
}

impl<K: Ord + Eq + Copy, V> Trie<K, V> {
    fn new() -> Self {
        Trie { root:  Node { key: None, value: None, next: vec![] } }
    }

    fn insert(&mut self, key: &[K], value: V) -> bool {
        fn _insert<K: Ord + Eq + Copy, V>(vec: &mut Vec<Node<K, V>>, new_node: Node<K, V>) -> usize {
            vec.push(new_node);
            let mut idx = vec.len() - 1;
            while idx > 0 {
                if vec[idx].key < vec[idx - 1].key {
                    vec.swap(idx, idx - 1);
                    idx -= 1;
                } else {
                    break;
                }
            }
            idx
        }

        let mut node = &mut self.root;
        for k in key {
            if let Some(idx) = Self::search(&node.next, &k) {
                node = &mut node.next[idx];
            } else {
                let new_node = Node { key: Some(*k), value: None, next: vec![] };
                let next_idx = _insert(&mut node.next, new_node);
                node = &mut node.next[next_idx];
            }
        }
        if node.value.is_none() {
            node.value = Some(value);
            true
        } else {
            false
        }
    }

    fn find(&self, key: &[K]) -> Option<&V> {
        let mut node = &self.root;
        for k in key {
            if let Some(idx) = Self::search(&node.next, &k) {
                node = &node.next[idx];
            } else {
                return None
            }
        }
        if let Some(ref value) = node.value {
            Some(value)
        } else {
            None
        }
    }

    fn search<'a>(nodes: &[Node<K, V>], key: &K) -> Option<usize> {
        if nodes.is_empty() {
            return None
        }
        let mut s = 0usize;
        let mut e = nodes.len() - 1;
        loop {
            if s > e {
                break;
            }
            let pivot = (s + e) / 2;
            let pivot_key = nodes[pivot].key.as_ref().unwrap();
            if key < pivot_key {
                e = pivot - 1;
            } else if key > pivot_key {
                s = pivot + 1;
            } else {
                return Some(pivot);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trie_1() {
        let mut trie: Trie<u8, String> = Trie::new();
        trie.insert(String::from("abc").as_bytes(), String::from("abc"));
        let s = String::from("abc");
        assert_eq!(Some(&s), trie.find(s.as_bytes()));
    }
}
