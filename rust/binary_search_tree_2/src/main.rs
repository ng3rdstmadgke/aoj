use std::fmt::Display;
use std::fmt::Write;

fn main() {
    let mut bst: BST<i32>  = BST::new();
    let n: i32 = read();
    for _ in 0..n {
        let vec: Vec<String> = read_as_vec();
        let cmd: &str = &vec[0];
        if cmd == "insert" {
            let key: i32 = vec[1].parse().ok().unwrap();
            bst.insert(key);
        } else if cmd == "find" {
            let key: i32 = vec[1].parse().ok().unwrap();
            if bst.find(key) {
                println!("yes");
            } else {
                println!("no");
            }
        } else if cmd == "delete" {
            let key: i32 = vec[1].parse().ok().unwrap();
            bst.delete(key);
        } else if cmd == "print" {
            println!(" {}", bst.inorder());
            println!(" {}", bst.preorder());
        }

    }
}

fn read<T: std::str::FromStr>() -> T {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<T>().ok().unwrap()
}

fn read_as_vec<T: std::str::FromStr>() -> Vec<T> {
    read::<String>()
        .split_whitespace()
        .map(|e| e.parse::<T>().ok().unwrap())
        .collect()
}

#[derive(Clone)]
struct Node<T: Clone + Ord + PartialOrd + Eq + PartialEq + Display> {
    value: T,
    left : Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

struct BST<T: Clone + Ord + PartialOrd + Eq + PartialEq + Display> {
    root: Option<Box<Node<T>>>,
}

impl<T: Clone + Ord + PartialOrd + Eq + PartialEq + Display> BST<T> {
    fn new() -> BST<T> {
        BST { root: None }
    }

    fn insert(&mut self, value: T) {
        Node::insert(&mut self.root, value);
    }

    fn find(&self, value: T) -> bool {
        Node::find(& self.root, value)
    }

    fn delete(&mut self, value: T) {
        Node::delete(&mut self.root, value)
    }

    fn preorder(&self) -> String{
        Node::preorder(& self.root)
    }

    fn inorder(&self) -> String {
        Node::inorder(& self.root)
    }
}

impl<T: Clone + Ord + PartialOrd + Eq + PartialEq + Display> Node<T> {
    fn new(value: T) -> Node<T> {
        Node { value: value, left : None, right: None }
    }

    fn insert(node: &mut Option<Box<Node<T>>>, value: T) {
        if let Some(ref mut box_node) = node {
            if value < box_node.value {
                Self::insert(&mut box_node.left, value);
            } else if value > box_node.value {
                Self::insert(&mut box_node.right, value);
            }
        } else {
            *node = Some(Box::new(Node::new(value)));

        }
    }

    fn find(node: & Option<Box<Node<T>>>, value: T) -> bool {
        if let Some(ref box_node) = node {
            if value < box_node.value {
                Self::find(& box_node.left, value)
            } else if value > box_node.value {
                Self::find(& box_node.right, value)
            } else {
                true
            }
        } else {
            false
        }
    }

    fn find2(node: &mut Option<Box<Node<T>>>, value: T) -> *mut Option<Box<Node<T>>> {
        if let Some(ref mut box_node) = node {
            if value < box_node.value {
                Self::find2(&mut box_node.left, value)
            } else if value > box_node.value {
                Self::find2(&mut box_node.right, value)
            } else {
                node
            }
        } else {
            node
        }
    }

    fn next_node(node: &mut Option<Box<Node<T>>>) -> Option<*mut Option<Box<Node<T>>>> {
        if let Some(ref mut box_node) = *node {
            if box_node.left.is_none() {
                Some(node)
            } else {
                Self::next_node(&mut box_node.left)
            }
        } else {
            None
        }
    }

    fn delete(node: &mut Option<Box<Node<T>>>, value: T) {
        unsafe fn sub<U>(node: *mut Option<Box<Node<U>>>)
            where U: Clone + Ord + PartialOrd + Eq + PartialEq + Display
        {
            if let Some(ref mut box_node) = *node {
                if box_node.left.is_none() && box_node.right.is_none() {
                    *node = None;
                } else if box_node.left.is_none() {
                    *node = box_node.right.clone();
                } else if box_node.right.is_none() {
                    *node = box_node.left.clone();
                } else {
                    let next_node = Node::next_node(&mut box_node.right).unwrap();
                    if let Some(ref mut next_box_node) = *next_node {
                        box_node.value = next_box_node.value.clone();
                    }
                    sub(next_node)
                }
            }

        }
        let target = Self::find2(node, value);
        unsafe {
            sub(target)
        }
    }

    fn preorder(node: & Option<Box<Node<T>>>) -> String {
        fn sub<U>(node: & Option<Box<Node<U>>>, buf: &mut String)
            where U: Clone + Ord + PartialOrd + Eq + PartialEq + Display
        {
            if let Some(ref box_node) = node {
                write!(buf, "{} ", box_node.value).unwrap();
                sub(& box_node.left, buf);
                sub(& box_node.right, buf);
            }
        }
        let mut buf = String::new();
        sub(& node, &mut buf);
        buf.pop();
        buf
    }

    fn inorder(node: & Option<Box<Node<T>>>) -> String {
        fn sub<U>(node: & Option<Box<Node<U>>>, buf: &mut String)
            where U: Clone + Ord + PartialOrd + Eq + PartialEq + Display
        {
            if let Some(ref box_node) = node {
                sub(& box_node.left, buf);
                write!(buf, "{} ", box_node.value).unwrap();
                sub(& box_node.right, buf);
            }
        }
        let mut buf = String::new();
        sub(& node, &mut buf);
        buf.pop();
        buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bst_1() {
        let mut bst: BST<u32> = BST::new();
        bst.insert(30);
        bst.insert(88);
        bst.insert(12);
        bst.insert(1);
        bst.insert(20);
        bst.insert(17);
        bst.insert(25);
        assert_eq!(true, bst.find(17));
        assert_eq!(false, bst.find(18));
        assert_eq!("1 12 17 20 25 30 88".to_string(), bst.inorder());
        assert_eq!("30 12 1 20 17 25 88".to_string(), bst.preorder());
    }

    #[test]
    fn test_bst_2() {
        let mut bst: BST<u32> = BST::new();
        bst.insert(8);
        bst.insert(2);
        bst.insert(3);
        bst.insert(7);
        bst.insert(22);
        bst.insert(1);
        assert_eq!(true, bst.find(1));
        assert_eq!(true, bst.find(2));
        assert_eq!(true, bst.find(3));
        assert_eq!(false, bst.find(4));
        assert_eq!(false, bst.find(5));
        assert_eq!(false, bst.find(6));
        assert_eq!(true, bst.find(7));
        assert_eq!(true, bst.find(8));
        assert_eq!("1 2 3 7 8 22".to_string(), bst.inorder());
        assert_eq!("8 2 1 3 7 22".to_string(), bst.preorder());
        bst.delete(1);
        assert_eq!("2 3 7 8 22".to_string(), bst.inorder());
        assert_eq!("8 2 3 7 22".to_string(), bst.preorder());
    }

    #[test]
    fn test_bst_3() {
        let mut bst: BST<u32> = BST::new();
        bst.insert(8);
        bst.insert(2);
        bst.insert(3);
        bst.insert(7);
        bst.insert(22);
        bst.insert(1);
        assert_eq!(true, bst.find(1));
        assert_eq!(true, bst.find(2));
        assert_eq!(true, bst.find(3));
        assert_eq!(false, bst.find(4));
        assert_eq!(false, bst.find(5));
        assert_eq!(false, bst.find(6));
        assert_eq!(true, bst.find(7));
        assert_eq!(true, bst.find(8));
        assert_eq!("1 2 3 7 8 22".to_string(), bst.inorder());
        assert_eq!("8 2 1 3 7 22".to_string(), bst.preorder());
        bst.delete(3);
        bst.delete(7);
        assert_eq!("1 2 8 22".to_string(), bst.inorder());
        assert_eq!("8 2 1 22".to_string(), bst.preorder());
    }

    #[test]
    fn test_bst_4() {
        let mut bst: BST<u32> = BST::new();
        bst.insert(8);
        bst.insert(2);
        bst.insert(3);
        bst.insert(7);
        bst.insert(22);
        bst.insert(1);
        assert_eq!(true, bst.find(1));
        assert_eq!(true, bst.find(2));
        assert_eq!(true, bst.find(3));
        assert_eq!(false, bst.find(4));
        assert_eq!(false, bst.find(5));
        assert_eq!(false, bst.find(6));
        assert_eq!(true, bst.find(7));
        assert_eq!(true, bst.find(8));
        assert_eq!("1 2 3 7 8 22".to_string(), bst.inorder());
        assert_eq!("8 2 1 3 7 22".to_string(), bst.preorder());
        bst.delete(2);
        assert_eq!("1 3 7 8 22".to_string(), bst.inorder());
        assert_eq!("8 3 1 7 22".to_string(), bst.preorder());
    }

    #[test]
    fn test_bst_5() {
        let mut bst: BST<u32> = BST::new();
        bst.insert(8);
        bst.insert(2);
        bst.insert(3);
        bst.insert(7);
        bst.insert(22);
        bst.insert(1);
        assert_eq!(true, bst.find(1));
        assert_eq!(true, bst.find(2));
        assert_eq!(true, bst.find(3));
        assert_eq!(false, bst.find(4));
        assert_eq!(false, bst.find(5));
        assert_eq!(false, bst.find(6));
        assert_eq!(true, bst.find(7));
        assert_eq!(true, bst.find(8));
        assert_eq!("1 2 3 7 8 22".to_string(), bst.inorder());
        assert_eq!("8 2 1 3 7 22".to_string(), bst.preorder());
        bst.delete(8);
        assert_eq!("1 2 3 7 22".to_string(), bst.inorder());
        assert_eq!("22 2 1 3 7".to_string(), bst.preorder());
    }

    #[test]
    fn test_bst_6() {
        let mut bst: BST<i32> = BST::new();
        bst.insert(30);
        bst.insert(17);
        bst.insert(88);
        bst.insert(53);
        bst.insert(5);
        bst.insert(20);
        bst.insert(18);
        bst.insert(28);
        bst.insert(27);
        bst.insert(60);
        bst.insert(2000000000);
        bst.insert(55);
        bst.insert(63);
        bst.insert(-1);
        bst.insert(8);
        bst.delete(53);
        assert_eq!("-1 5 8 17 18 20 27 28 30 55 60 63 88 2000000000".to_string(), bst.inorder());
        bst.delete(2000000000);
        assert_eq!("-1 5 8 17 18 20 27 28 30 55 60 63 88".to_string(), bst.inorder());
        bst.delete(20);
        assert_eq!("-1 5 8 17 18 27 28 30 55 60 63 88".to_string(), bst.inorder());
        bst.delete(5);
        assert_eq!("-1 8 17 18 27 28 30 55 60 63 88".to_string(), bst.inorder());
        bst.delete(8);
        assert_eq!("-1 17 18 27 28 30 55 60 63 88".to_string(), bst.inorder());
        assert_eq!("30 17 -1 27 18 28 88 60 55 63".to_string(), bst.preorder());
    }
}
