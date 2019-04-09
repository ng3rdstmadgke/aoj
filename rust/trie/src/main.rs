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

    /// trie に key => value を挿入する。
    fn insert(&mut self, key: &[K], value: V) -> bool {
        /// 挿入ソートで trie に key => value を挿入する。
        fn _insert<K: Ord + Eq + Copy, V>(vec: &mut Vec<Node<K, V>>, new_node: Node<K, V>) -> usize {
            vec.push(new_node);
            let mut idx = vec.len() - 1;
            while idx > 0 {
                if vec[idx - 1].key > vec[idx].key {
                    vec.swap(idx - 1, idx);
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
                // 辿れる場合は辿る
                node = &mut node.next[idx];
            } else {
                // 辿れなくなったら新しいノードを挿入
                let new_node = Node { key: Some(*k), value: None, next: vec![] };
                let next_idx = _insert(&mut node.next, new_node);
                node = &mut node.next[next_idx];
            }
        }
        // すでにkeyに対応するvalueが存在している場合は更新せずにfalseを返す
        if node.value.is_some() {
            return false;
        }
        node.value = Some(value);
        true
    }

    /// trie から key を探索する
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

    /// Node の配列から key を２分探索で探索する。
    fn search(nodes: &[Node<K, V>], key: &K) -> Option<usize> {
        if nodes.is_empty() {
            return None
        }
        let mut s = 0usize;
        let mut e = nodes.len();
        loop {
            if s >= e {
                break;
            }
            let pivot = (s + e) / 2;
            let pivot_key = nodes[pivot].key.as_ref().unwrap();
            if key < pivot_key {
                e = pivot;
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
        let s = String::from("cba");
        assert_eq!(None, trie.find(s.as_bytes()));
    }

    #[test]
    fn test_trie_2() {
        let mut trie: Trie<u8, u32> = Trie::new();
        trie.insert(String::from("abc").as_bytes(), 0);
        trie.insert(String::from("abd").as_bytes(), 1);
        trie.insert(String::from("zyx").as_bytes(), 2);
        trie.insert(String::from("zwx").as_bytes(), 3);
        let s = String::from("abc");
        assert_eq!(Some(&0), trie.find(s.as_bytes()));
        let s = String::from("abd");
        assert_eq!(Some(&1), trie.find(s.as_bytes()));
        let s = String::from("zyx");
        assert_eq!(Some(&2), trie.find(s.as_bytes()));
        let s = String::from("zwx");
        assert_eq!(Some(&3), trie.find(s.as_bytes()));
    }

    #[test]
    fn test_trie_3() {
        let mut trie: Trie<u8, u32> = Trie::new();
        trie.insert(String::from("あいうえお").as_bytes(), 10);
        trie.insert(String::from("あいえうお").as_bytes(), 11);
        trie.insert(String::from("漢字").as_bytes()      , 12);
        trie.insert(String::from("平仮名").as_bytes()    , 13);
        trie.insert(String::from("片仮名").as_bytes()    , 14);
        let s = String::from("あいうえお");
        assert_eq!(Some(&10), trie.find(s.as_bytes()));
        let s = String::from("あいえうお");
        assert_eq!(Some(&11), trie.find(s.as_bytes()));
        let s = String::from("漢字");
        assert_eq!(Some(&12), trie.find(s.as_bytes()));
        let s = String::from("平仮名");
        assert_eq!(Some(&13), trie.find(s.as_bytes()));
        let s = String::from("片仮名");
        assert_eq!(Some(&14), trie.find(s.as_bytes()));
    }
}
