use std::fmt::Write;
use Node::{Nil, Cons};
use std::fmt::Display;

fn main() {
    let bst: BST<u32>  = BST::new();
}

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq)]
enum Node<T: Clone + Ord + PartialOrd + Eq + PartialEq + Display> {
    Nil,
    Cons(T, Box<Node<T>>, Box<Node<T>>),
}

struct BST<T: Clone + Ord + PartialOrd + Eq + PartialEq + Display> {
    root: Node<T>
}

impl<T: Clone + Ord + PartialOrd + Eq + PartialEq + Display> BST<T> {
    fn new() -> BST<T> {
        BST { root: Nil }
    }

    fn find(&mut self, target: T) -> bool {
        self.root.find(target).is_some()
    }

    fn insert(&mut self, value: T) {
        self.root.insert(value);
    }

    fn preorder(&self) -> String {
        let mut buf: String = String::new();
        self.root.preorder(&mut buf);
        buf.pop();
        buf
    }

    fn inorder(&self) -> String {
        let mut buf: String = String::new();
        self.root.inorder(&mut buf);
        buf.pop();
        buf
    }
}

impl<T: Clone + Ord + PartialOrd + Eq + PartialEq + Display> Node<T> {
    fn insert(&mut self, value: T) {
        if let Cons(ref v, ref mut l, ref mut r) = *self {
            if value < *v {
                l.insert(value);
            } else if value > *v {
                r.insert(value);
            }
        } else {
            *self = Cons(value, Box::new(Nil), Box::new(Nil));
        }
    }

    fn find(&mut self, target: T) -> Option<*mut Self> {
        if let Cons(ref v, ref mut l, ref mut r) = *self {
            if target < *v {
                l.find(target)
            } else if target > *v {
                r.find(target)
            } else {
                Some(self as *mut Self)
            }
        } else {
            None
        }
    }

    fn delete(&mut self, target: T) {
        if let Some(raw_ptr) = self.find(target) {
            unsafe {
                if let Cons(ref v, ref mut l, ref mut r) = *raw_ptr {
                    if **l == Nil && **r == Nil {
                        *raw_ptr = Nil;
                    } else if **l == Nil {
                        *raw_ptr = *r.clone();
                    } else if **r == Nil {
                        *raw_ptr = *l.clone();
                    } else {
                        if let Cons(ref cv, ref mut cl, ref mut cr) = **r {
                            *raw_ptr = Cons(cv.clone(), *l, *r);
                        }
                    }
                }
            }

        }

    }

    /*
    fn sub(&mut self) {
        if let Cons(ref v, ref mut l, ref mut r) = *self {
            match (**l, **r) {
                (Nil, Nil) => {
                }
                (Nil, _) | (_, Nil) => {
                }
                (_, Cons(ref vv, ref mut ll, ref mut rr)) => {
                }
            }
        }
    }
    */

    fn preorder(&self, buf: &mut String) {
        if let Cons(ref v, ref l, ref r) = *self {
            write!(buf, "{} ", v).unwrap();
            l.preorder(buf);
            r.preorder(buf);
        }
    }

    fn inorder(&self, buf: &mut String) {
        if let Cons(ref v, ref l, ref r) = *self {
            l.inorder(buf);
            write!(buf, "{} ", v).unwrap();
            r.inorder(buf);
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
}
