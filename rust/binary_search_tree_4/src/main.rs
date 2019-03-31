use std::fmt::Write;
use std::mem;

fn main() {
}

struct Node {
    key: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

struct Tree {
    root: Option<Box<Node>>,
}

impl Tree {
    fn new() -> Tree {
        Tree { root: None }
    }

    fn insert(&mut self, key: i32) {
        fn sub(node: &mut Option<Box<Node>>, key: i32) {
            if let Some(ref mut unwraped) = node {
                if key < unwraped.key {
                    sub(&mut unwraped.left, key);
                } else if key > unwraped.key {
                    sub(&mut unwraped.right, key);
                } else {
                }
            } else {
                *node = Some(Box::new(Node { key: key, left: None, right: None }));
            }
        }
        sub(&mut self.root, key);
    }

    fn find(&self, key: i32) -> bool {
        fn sub(node: &Option<Box<Node>>, key: i32) -> bool {
            if let Some(ref unwraped) = node {
                if key < unwraped.key {
                    sub(&unwraped.left, key)
                } else if key > unwraped.key {
                    sub(&unwraped.right, key)
                } else {
                    true
                }
            } else {
                false
            }
        }
        sub(&self.root, key)
    }

    fn delete(&mut self, key: i32) {
        fn sub(node: &mut Option<Box<Node>>, key: i32) {
            if let Some(ref mut unwraped) = node {
                if key < unwraped.key {
                    sub(&mut unwraped.left, key);
                } else if key > unwraped.key {
                    sub(&mut unwraped.right, key);
                } else {
                    if unwraped.left.is_some() && unwraped.right.is_some() {
                        // 1. inorderで次ノードを探索する
                        // 2. 次ノードのkeyを削除ノードのkeyにコピーする
                        // 3. 次ノードを削除する
                        let next_node = find_next(&mut unwraped.right);
                        let next_key = next_node.as_ref().unwrap().key;
                        unwraped.key = next_key;
                        sub(next_node, next_key);
                    } else if unwraped.left.is_some() {
                        *node = mem::replace(&mut unwraped.left, None);
                    } else if unwraped.right.is_some() {
                        *node = mem::replace(&mut unwraped.right, None);
                    } else {
                        *node = None;
                    }
                }

            }
        }
        /// inorder で次ノードを探索する
        fn find_next(node: &mut Option<Box<Node>>) -> &mut Option<Box<Node>> {
            if node.as_mut().unwrap().left.is_none() {
                node
            } else {
                find_next(&mut node.as_mut().unwrap().left)
            }
        }
        sub(&mut self.root, key);
    }

    fn preorder(&self) -> String {
        fn sub(node: &Option<Box<Node>>, buf: &mut String) {
            if let Some(ref unwraped) = node {
                write!(buf, "{} ", unwraped.key).unwrap();
                sub(&unwraped.left, buf);
                sub(&unwraped.right, buf);
            }
        }
        let mut buf = String::new();
        sub(&self.root, &mut buf);
        buf.pop();
        buf
    }

    fn inorder(&self) -> String {
        fn sub(node: &Option<Box<Node>>, buf: &mut String) {
            if let Some(ref unwraped) = node {
                sub(&unwraped.left, buf);
                write!(buf, "{} ", unwraped.key).unwrap();
                sub(&unwraped.right, buf);
            }
        }
        let mut buf = String::new();
        sub(&self.root, &mut buf);
        buf.pop();
        buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bst_1() {
        let mut tree: Tree = Tree::new();
        tree.insert(30);
        tree.insert(88);
        tree.insert(12);
        tree.insert(1);
        tree.insert(20);
        tree.insert(17);
        tree.insert(25);
        assert_eq!(true, tree.find(17));
        assert_eq!(false, tree.find(18));
        assert_eq!("1 12 17 20 25 30 88".to_string(), tree.inorder());
        assert_eq!("30 12 1 20 17 25 88".to_string(), tree.preorder());
    }

    #[test]
    fn test_tree_2() {
        let mut tree: Tree = Tree::new();
        tree.insert(8);
        tree.insert(2);
        tree.insert(3);
        tree.insert(7);
        tree.insert(22);
        tree.insert(1);
        assert_eq!(true, tree.find(1));
        assert_eq!(true, tree.find(2));
        assert_eq!(true, tree.find(3));
        assert_eq!(false, tree.find(4));
        assert_eq!(false, tree.find(5));
        assert_eq!(false, tree.find(6));
        assert_eq!(true, tree.find(7));
        assert_eq!(true, tree.find(8));
        assert_eq!("1 2 3 7 8 22".to_string(), tree.inorder());
        assert_eq!("8 2 1 3 7 22".to_string(), tree.preorder());
        tree.delete(1);
        assert_eq!("2 3 7 8 22".to_string(), tree.inorder());
        assert_eq!("8 2 3 7 22".to_string(), tree.preorder());
    }

    #[test]
    fn test_tree_3() {
        let mut tree: Tree = Tree::new();
        tree.insert(8);
        tree.insert(2);
        tree.insert(3);
        tree.insert(7);
        tree.insert(22);
        tree.insert(1);
        assert_eq!(true, tree.find(1));
        assert_eq!(true, tree.find(2));
        assert_eq!(true, tree.find(3));
        assert_eq!(false, tree.find(4));
        assert_eq!(false, tree.find(5));
        assert_eq!(false, tree.find(6));
        assert_eq!(true, tree.find(7));
        assert_eq!(true, tree.find(8));
        assert_eq!("1 2 3 7 8 22".to_string(), tree.inorder());
        assert_eq!("8 2 1 3 7 22".to_string(), tree.preorder());
        tree.delete(3);
        tree.delete(7);
        assert_eq!("1 2 8 22".to_string(), tree.inorder());
        assert_eq!("8 2 1 22".to_string(), tree.preorder());
    }

    #[test]
    fn test_tree_4() {
        let mut tree: Tree = Tree::new();
        tree.insert(8);
        tree.insert(2);
        tree.insert(3);
        tree.insert(7);
        tree.insert(22);
        tree.insert(1);
        assert_eq!(true, tree.find(1));
        assert_eq!(true, tree.find(2));
        assert_eq!(true, tree.find(3));
        assert_eq!(false, tree.find(4));
        assert_eq!(false, tree.find(5));
        assert_eq!(false, tree.find(6));
        assert_eq!(true, tree.find(7));
        assert_eq!(true, tree.find(8));
        assert_eq!("1 2 3 7 8 22".to_string(), tree.inorder());
        assert_eq!("8 2 1 3 7 22".to_string(), tree.preorder());
        tree.delete(2);
        assert_eq!("1 3 7 8 22".to_string(), tree.inorder());
        assert_eq!("8 3 1 7 22".to_string(), tree.preorder());
    }

    #[test]
    fn test_tree_5() {
        let mut tree: Tree = Tree::new();
        tree.insert(8);
        tree.insert(2);
        tree.insert(3);
        tree.insert(7);
        tree.insert(22);
        tree.insert(1);
        assert_eq!(true, tree.find(1));
        assert_eq!(true, tree.find(2));
        assert_eq!(true, tree.find(3));
        assert_eq!(false, tree.find(4));
        assert_eq!(false, tree.find(5));
        assert_eq!(false, tree.find(6));
        assert_eq!(true, tree.find(7));
        assert_eq!(true, tree.find(8));
        assert_eq!("1 2 3 7 8 22".to_string(), tree.inorder());
        assert_eq!("8 2 1 3 7 22".to_string(), tree.preorder());
        tree.delete(8);
        assert_eq!("1 2 3 7 22".to_string(), tree.inorder());
        assert_eq!("22 2 1 3 7".to_string(), tree.preorder());
    }

    #[test]
    fn test_tree_6() {
        let mut tree: Tree = Tree::new();
        tree.insert(30);
        tree.insert(17);
        tree.insert(88);
        tree.insert(53);
        tree.insert(5);
        tree.insert(20);
        tree.insert(18);
        tree.insert(28);
        tree.insert(27);
        tree.insert(60);
        tree.insert(2000000000);
        tree.insert(55);
        tree.insert(63);
        tree.insert(-1);
        tree.insert(8);
        tree.delete(53);
        assert_eq!("-1 5 8 17 18 20 27 28 30 55 60 63 88 2000000000".to_string(), tree.inorder());
        tree.delete(2000000000);
        assert_eq!("-1 5 8 17 18 20 27 28 30 55 60 63 88".to_string(), tree.inorder());
        tree.delete(20);
        assert_eq!("-1 5 8 17 18 27 28 30 55 60 63 88".to_string(), tree.inorder());
        tree.delete(5);
        assert_eq!("-1 8 17 18 27 28 30 55 60 63 88".to_string(), tree.inorder());
        tree.delete(8);
        assert_eq!("-1 17 18 27 28 30 55 60 63 88".to_string(), tree.inorder());
        assert_eq!("30 17 -1 27 18 28 88 60 55 63".to_string(), tree.preorder());
    }
}
