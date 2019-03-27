use std::fmt::Write;
use std::mem;

fn main() {
    let mut treap = Treap::new();
    let n: usize = read();
    for _ in 0..n {
        let input: Vec<String> = read_as_vec();
        match &input[0][..] {
            "insert" => {
                let key: u32 = input[1].parse().ok().unwrap();
                let pri: u32 = input[2].parse().ok().unwrap();
                treap.insert(key, pri);
            },
            "find" => {
                let key: u32 = input[1].parse().ok().unwrap();
                if treap.find(key) {
                    println!("yes");
                } else {
                    println!("no");
                }
            },
            "delete" => {
                let key: u32 = input[1].parse().ok().unwrap();
                treap.delete(key);
            },
            "print" => {
                println!(" {}", treap.inorder());
                println!(" {}", treap.preorder());
            },
            _ => { },
        }
    }

}

struct Node {
    key   : u32,
    pri   : u32,
    left  : Option<Box<Node>>,
    right : Option<Box<Node>>,
}

struct Treap {
    root: Option<Box<Node>>,
}

impl Treap {
    fn new() -> Self {
        Treap { root: None }
    }

    fn insert(&mut self, key: u32, pri: u32) {
        Self::_insert(&mut self.root, key, pri);
    }

    fn _insert(node_ptr: &mut Option<Box<Node>>, key: u32, pri: u32) {
        if let Some(ref mut node) = node_ptr {
            if key < node.key {
                Self::_insert(&mut node.left, key, pri);
                if node.pri < node.left.as_ref().unwrap().pri {
                    Self::right_rotate(node_ptr);
                    /*
                    let mut left: Option<Box<Node>> = mem::replace(&mut node.left, None);
                    node.left = mem::replace(&mut left.as_mut().unwrap().right, None);
                    left.as_mut().unwrap().right =  mem::replace(node_ptr, None);
                    *node_ptr = left;
                    */
                }
            } else if key > node.key {
                Self::_insert(&mut node.right, key, pri);
                if node.pri < node.right.as_ref().unwrap().pri {
                    Self::left_rotate(node_ptr);
                    /*
                    let mut right: Option<Box<Node>> = mem::replace(&mut node.right, None);
                    node.right = mem::replace(&mut right.as_mut().unwrap().left, None);
                    right.as_mut().unwrap().left = mem::replace(node_ptr, None);
                    *node_ptr = right;
                    */
                }
            }
        } else {
            *node_ptr = Some(Box::new(Node { key: key, pri: pri, left: None, right: None }));
        }
    }

    /// 右回転(左の子が親になる)
    fn right_rotate(node_ptr: &mut Option<Box<Node>>) {
        let node = node_ptr.as_mut().unwrap();
        let mut left: Option<Box<Node>> = mem::replace(&mut node.left, None);
        // 親の左の子に、左の子の右の子をつける
        node.left = mem::replace(&mut left.as_mut().unwrap().right, None);
        // 左の子の右の子に、親をつける
        left.as_mut().unwrap().right =  mem::replace(node_ptr, None);
        // 親の位置に左の子を持ってくる
        *node_ptr = left;
    }

    /// 左回転(右の子が親になる)
    fn left_rotate(node_ptr: &mut Option<Box<Node>>) {
        let node = node_ptr.as_mut().unwrap();
        let mut right: Option<Box<Node>> = mem::replace(&mut node.right, None);
        // 親の右の子に、右の子の左の子をつける
        node.right = mem::replace(&mut right.as_mut().unwrap().left, None);
        // 右の子の左の子に、親をつける
        right.as_mut().unwrap().left = mem::replace(node_ptr, None);
        // 親の位置に右の子を持ってくる
        *node_ptr = right;
    }


    fn find(&self, key: u32) -> bool{
        fn sub(node: &Option<Box<Node>>, key: u32) -> bool {
            if let Some(ref node) = node {
                if key < node.key  {
                    sub(&node.left, key)
                } else if key > node.key {
                    sub(&node.right, key)
                } else {
                    true
                }
            } else {
                false
            }
        }
        sub(&self.root, key)
    }

    fn delete(&mut self, key: u32) {
    }

    fn preorder(&self) -> String {
        fn sub(node: &Option<Box<Node>>, buf: &mut String) {
            if let Some(ref n) = node {
                write!(buf, "{} ", n.key).unwrap();
                sub(&n.left, buf);
                sub(&n.right, buf);
            }
        }
        let mut buf = String::new();
        sub(&self.root, &mut buf);
        buf.pop();
        buf
    }

    fn inorder(&self) -> String {
        fn sub(node: &Option<Box<Node>>, buf: &mut String) {
            if let Some(ref n) = node {
                sub(&n.left, buf);
                write!(buf, "{} ", n.key).unwrap();
                sub(&n.right, buf);
            }
        }
        let mut buf = String::new();
        sub(&self.root, &mut buf);
        buf.pop();
        buf
    }
}

fn read<T: std::str::FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.parse::<T>().ok().unwrap()
}

fn read_as_vec<T: std::str::FromStr>() -> Vec<T> {
    read::<String>()
        .split_whitespace()
        .map(|e| { e.parse::<T>().ok().unwrap() })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_1() {
        let mut treap = Treap::new();
        treap.insert(20, 99);
        treap.insert(10, 50);
        treap.insert(30, 80);
        //assert_eq!(String::from("10 20 30"), treap.inorder());
        //assert_eq!(String::from("20 10 30"), treap.preorder());
    }

    #[test]
    fn test_insert_2() {
        let mut treap = Treap::new();
        treap.insert(20, 60);
        treap.insert(10, 50);
        treap.insert(30, 40);
        treap.insert(6, 90);
        assert_eq!(String::from("6 20 10 30"), treap.preorder());
        assert_eq!(String::from("6 10 20 30"), treap.inorder());
    }

    #[test]
    fn test_insert_3() {
        let mut treap = Treap::new();
        treap.insert(20, 60);
        treap.insert(10, 50);
        treap.insert(30, 40);
        treap.insert(40, 90);
        assert_eq!(String::from("40 20 10 30"), treap.preorder());
        assert_eq!(String::from("10 20 30 40"), treap.inorder());
    }

    #[test]
    fn test_insert_4() {
        let mut treap = Treap::new();
        treap.insert(35, 99);
        treap.insert(3 , 80);
        treap.insert(1 , 53);
        treap.insert(14, 25);
        treap.insert(7 , 10);
        treap.insert(21, 12);
        treap.insert(80, 76);
        treap.insert(42, 3);
        treap.insert(86, 47);
        assert_eq!(String::from("1 3 7 14 21 35 42 80 86"), treap.inorder());
        assert_eq!(String::from("35 3 1 14 7 21 80 42 86"), treap.preorder());
        treap.insert(6, 90);
        assert_eq!(String::from("35 6 3 1 14 7 21 80 42 86"), treap.preorder());
        assert_eq!(String::from("1 3 6 7 14 21 35 42 80 86"), treap.inorder());
    }

    #[test]
    fn test_insert_5() {
        let mut treap = Treap::new();
        treap.insert(35, 99);
        treap.insert(3 , 80);
        treap.insert(1 , 53);
        treap.insert(14, 25);
        treap.insert(7 , 10);
        treap.insert(21, 12);
        treap.insert(80, 76);
        treap.insert(42, 3);
        treap.insert(86, 47);
        treap.insert(6, 90);
        assert_eq!(true, treap.find(35));
        assert_eq!(true, treap.find(3));
        assert_eq!(true, treap.find(1));
        assert_eq!(true, treap.find(14));
        assert_eq!(true, treap.find(7));
        assert_eq!(true, treap.find(21));
        assert_eq!(true, treap.find(80));
        assert_eq!(true, treap.find(42));
        assert_eq!(true, treap.find(86));
        assert_eq!(true, treap.find(6));
        assert_eq!(false, treap.find(10));
    }
}
