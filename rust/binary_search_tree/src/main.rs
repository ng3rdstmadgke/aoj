use std::fmt::Write;
use Node::{Nil, Cons};
use std::fmt::Display;
use std::cell::RefCell;
use std::rc::Rc;

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

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
enum Node<T: Ord + PartialOrd + Eq + PartialEq  + Display> {
    Nil,
    Cons(
        Rc<RefCell<T>>,
        Rc<RefCell<Node<T>>>,
        Rc<RefCell<Node<T>>>,
        ),
}

struct BST<T: Ord + PartialOrd + Eq + PartialEq + Display> {
    root: Rc<RefCell<Node<T>>>
}

impl<T: Ord + PartialOrd + Eq + PartialEq + Display> BST<T> {
    fn new() -> BST<T> {
        BST { root: Rc::new(RefCell::new(Nil)) }
    }

    fn find(&mut self, target: T) -> bool {
        Node::find(Rc::clone(& self.root), target).is_some()
    }

    fn insert(&mut self, value: T) {
        Node::insert(Rc::clone(& self.root), value);
    }

    fn delete(&mut self, target: T) {
        if let Some((ptr, node)) = Node::find_for_delete(&mut self.root, Rc::clone(& self.root), target) {
            Node::delete(ptr, node);
        }
    }

    fn preorder(&self) -> String {
        let mut buf: String = String::new();
        Node::preorder(Rc::clone(& self.root), &mut buf);
        buf.pop();
        buf
    }

    fn inorder(&self) -> String {
        let mut buf: String = String::new();
        Node::inorder(Rc::clone(& self.root), &mut buf);
        buf.pop();
        buf
    }
}

impl<T: Ord + PartialOrd + Eq + PartialEq + Display> Node<T> {
    fn insert(node: Rc<RefCell<Node<T>>>, value: T) {
        match *node.borrow_mut() {
            Cons(ref v, ref l, ref r) => {
                if value < *v.borrow() {
                    Self::insert(Rc::clone(l), value);
                } else if value > *v.borrow() {
                    Self::insert(Rc::clone(r), value);
                }
            }, ref mut nil => { *nil = Cons(
                    Rc::new(RefCell::new(value)),
                    Rc::new(RefCell::new(Nil)),
                    Rc::new(RefCell::new(Nil)),
                    );
            }
        }
    }

    fn find(node: Rc<RefCell<Node<T>>>, target: T) -> Option<Rc<RefCell<Node<T>>>> {
        if let Cons(ref v, ref l, ref r) = *node.borrow() {
            if target < *v.borrow() {
                Self::find(Rc::clone(l), target)
            } else if target > *v.borrow() {
                Self::find(Rc::clone(r), target)
            } else {
                Some(Rc::clone(& node))
            }
        } else {
            None
        }
    }

    fn find_for_delete(ptr: *mut Rc<RefCell<Node<T>>>, node: Rc<RefCell<Node<T>>>, target: T) ->
        Option<(*mut Rc<RefCell<Node<T>>>, Rc<RefCell<Node<T>>>)> {
        if let Cons(ref v, ref mut l, ref mut r) = *node.borrow_mut() {
            if target < *v.borrow() {
                Self::find_for_delete(l, Rc::clone(l), target)
            } else if target > *v.borrow() {
                Self::find_for_delete(r, Rc::clone(r), target)
            } else {
                Some((ptr, Rc::clone(& node)))
            }
        } else {
            None
        }
    }

    fn find_next_node(ptr: *mut Rc<RefCell<Node<T>>>, node: Rc<RefCell<Node<T>>>) ->
        Option<(*mut Rc<RefCell<Node<T>>>, Rc<RefCell<Node<T>>>)> {
        if let Cons(_, ref mut l, _) = *node.borrow_mut() {
            if *l.borrow() == Nil {
                Some((ptr, Rc::clone(& node)))
            } else {
                Self::find_next_node(l, Rc::clone(l))
            }
        } else {
            None
        }
    }


    fn delete(ptr: *mut Rc<RefCell<Node<T>>>, node: Rc<RefCell<Node<T>>>) {
        let replace: Option<Rc<RefCell<Node<T>>>> = match *node.borrow() {
            Cons(_, ref l, ref r) => {
                if *l.borrow() == Nil && *r.borrow() == Nil {
                    Some(Rc::new(RefCell::new(Nil)))
                } else if *l.borrow() == Nil {
                    Some(Rc::clone(r))
                } else if *r.borrow() == Nil {
                    Some(Rc::clone(l))
                } else {
                    None
                }
            },
            Nil => None, // unreachable
        };
        if let Some(replace) = replace {
            unsafe {
                *ptr = replace;
            }
        } else {
            if let Cons(ref mut v, _, ref mut r) = *node.borrow_mut() {
                let (next_ptr, next_node) = Self::find_next_node(r, Rc::clone(r)).unwrap();
                if let Cons(ref cv, ..) = *next_node.borrow() {
                    *v = Rc::clone(cv);
                }
                Node::delete(next_ptr, next_node);
            }
        }
    }

    fn preorder(node: Rc<RefCell<Node<T>>>, buf: &mut String) {
        if let Cons(ref v, ref l, ref r) = *node.borrow() {
            write!(buf, "{} ", *v.borrow()).unwrap();
            Self::preorder(Rc::clone(l), buf);
            Self::preorder(Rc::clone(r), buf);
        }
    }

    fn inorder(node: Rc<RefCell<Node<T>>>, buf: &mut String) {
        if let Cons(ref v, ref l, ref r) = *node.borrow() {
            Self::inorder(Rc::clone(l), buf);
            write!(buf, "{} ", *v.borrow()).unwrap();
            Self::inorder(Rc::clone(r), buf);
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
