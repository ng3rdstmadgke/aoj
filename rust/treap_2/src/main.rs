use std::rc::Rc;
use std::cell::RefCell;
use std::fmt::Write;
use Node::{Nil, Cons};

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

enum Node {
    Nil,
    Cons(u32, u32, Rc<RefCell<Node>>, Rc<RefCell<Node>>),
}

impl Node {
    fn get_key(node: Rc<RefCell<Node>>) -> Option<u32> {
        if let Cons(k, _, _, _) = *node.borrow() { Some(k) } else { None }
    }

    fn get_pri(node: Rc<RefCell<Node>>) -> Option<u32> {
        if let Cons(_, p, _, _) = *node.borrow() { Some(p) } else { None }
    }

    fn get_left(node: Rc<RefCell<Node>>) -> Option<Rc<RefCell<Node>>> {
        if let Cons(_, _, ref l, _) = *node.borrow() { Some(Rc::clone(l)) } else { None }
    }

    fn get_right(node: Rc<RefCell<Node>>) -> Option<Rc<RefCell<Node>>> {
        if let Cons(_, _, _, ref r) = *node.borrow() { Some(Rc::clone(r)) } else { None }
    }
    fn get_all(node: Rc<RefCell<Node>>) -> Option<(u32, u32, Rc<RefCell<Node>>, Rc<RefCell<Node>>)> {
        if let Cons(k, p, ref l, ref r) = *node.borrow() { Some((k, p, Rc::clone(l), Rc::clone(r))) } else { None }
    }

    fn set_right(node: Rc<RefCell<Node>>, right: Rc<RefCell<Node>>) {
        if let Cons(_, _, _, ref mut r) = *node.borrow_mut() { *r = right; }
    }

    fn set_left(node: Rc<RefCell<Node>>, left: Rc<RefCell<Node>>) {
        if let Cons(_, _, ref mut l, _) = *node.borrow_mut() { *l = left; }
    }
}



struct Treap {
    root: Rc<RefCell<Node>>,
}

impl Treap {
    fn new() -> Self {
        Treap { root: Rc::new(RefCell::new(Nil)) }
    }

    fn right_rotate(sub_root: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
        let left = Node::get_left(Rc::clone(&sub_root)).unwrap();
        // 親の左の子に、左の子の右の子をつける
        if let Cons(_, _, _, ref r) = *left.borrow() {
            Node::set_left(Rc::clone(&sub_root), Rc::clone(r));
        }
        // 左の子の右の子に、親をつける
        Node::set_right(Rc::clone(&left), Rc::clone(&sub_root));
        left
    }
    
    fn left_rotate(sub_root: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
        let right = Node::get_right(Rc::clone(&sub_root)).unwrap();
        // 親の右の子に、右の子の左の子をつける
        if let Cons(_, _, ref l, _) = *right.borrow() {
            Node::set_right(Rc::clone(&sub_root), Rc::clone(l));
        }
        // 右の子の左の子に、親をつける
        Node::set_left(Rc::clone(&right), Rc::clone(&sub_root));
        right
    }

    fn insert(&mut self, key: u32, pri: u32) {
        self.root = Self::_insert(Rc::clone(& self.root), key, pri);
    }

    fn _insert(sub_root: Rc<RefCell<Node>>, key: u32, pri: u32) -> Rc<RefCell<Node>> {
        if let Nil = *sub_root.borrow() {
            let l = Rc::new(RefCell::new(Nil));
            let r = Rc::new(RefCell::new(Nil));
            return Rc::new(RefCell::new(Cons(key, pri, l, r)));
        }

        let (curr_key, curr_pri, curr_left, curr_right) = Node::get_all(Rc::clone(&sub_root)).unwrap();
        if key < curr_key { // 左の子へ移動
            let insert_node = Self::_insert(Rc::clone(&curr_left), key, pri);
            let left_pri = Node::get_pri(Rc::clone(&insert_node)).unwrap(); // curr_leftから取得してはいけない。(古いため)
            Node::set_left(Rc::clone(&sub_root), insert_node);
            if left_pri > curr_pri {
                // 左の子のほうが親よりも優先度が高い場合は右回転
                return Self::right_rotate(Rc::clone(&sub_root));
            }
        } else if key > curr_key { // 右の子へ移動
            let insert_node = Self::_insert(Rc::clone(&curr_right), key, pri);
            let right_pri = Node::get_pri(Rc::clone(&insert_node)).unwrap();
            Node::set_right(Rc::clone(&sub_root), insert_node);
            if right_pri > curr_pri {
                return Self::left_rotate(Rc::clone(&sub_root));
            }
        }
        sub_root
    }

    fn find(&self, key: u32) -> bool{
        true
    }

    fn delete(&mut self, key: u32) {
    }

    fn preorder(&self) -> String {
        fn sub(sub_root: Rc<RefCell<Node>>, buf: &mut String) {
            if let Cons(k, _, ref l, ref r) = *sub_root.borrow() {
                write!(buf, "{} ", k).unwrap();
                sub(Rc::clone(l), buf);
                sub(Rc::clone(r), buf);
            }
        }
        let mut buf = String::new();
        sub(Rc::clone(&self.root), &mut buf);
        buf.pop();
        buf
    }

    fn inorder(&self) -> String {
        fn sub(sub_root: Rc<RefCell<Node>>, buf: &mut String) {
            if let Cons(k, _, ref l, ref r) = *sub_root.borrow() {
                sub(Rc::clone(l), buf);
                write!(buf, "{} ", k).unwrap();
                sub(Rc::clone(r), buf);
            }
        }
        let mut buf = String::new();
        sub(Rc::clone(&self.root), &mut buf);
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
}
