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

    /// 葉にノードを挿入して、挿入したノードを適切な位置まで回転して移動させる
    fn _insert(node: &mut Option<Box<Node>>, key: u32, pri: u32) {
        if let Some(ref mut unwraped) = node {
            if key < unwraped.key {
                Self::_insert(&mut unwraped.left, key, pri);
                if unwraped.pri < unwraped.left.as_ref().unwrap().pri {
                    Self::right_rotate(node);
                }
            } else if key > unwraped.key {
                Self::_insert(&mut unwraped.right, key, pri);
                if unwraped.pri < unwraped.right.as_ref().unwrap().pri {
                    Self::left_rotate(node);
                }
            }
        } else {
            *node = Some(Box::new(Node { key: key, pri: pri, left: None, right: None }));
        }
    }

    /// 右回転(左の子が親になる)
    fn right_rotate(node: &mut Option<Box<Node>>) {
        let unwraped: &mut Box<Node> = node.as_mut().unwrap();
        let mut left: Option<Box<Node>> = mem::replace(&mut unwraped.left, None);
        if let Some(ref mut left) = left {
            // 親の左の子に、左の子の右の子をつける
            unwraped.left = mem::replace(&mut left.right, None);
            // 左の子の右の子に、親をつける
            left.right =  mem::replace(node, None);
        }
        // 親の位置に左の子を持ってくる
        *node = left;
    }

    /// 左回転(右の子が親になる)
    fn left_rotate(node: &mut Option<Box<Node>>) {
        let unwraped: &mut Box<Node> = node.as_mut().unwrap();
        let mut right: Option<Box<Node>> = mem::replace(&mut unwraped.right, None);
        if let Some(ref mut right) = right {
            // 親の右の子に、右の子の左の子をつける
            unwraped.right = mem::replace(&mut right.left, None);
            // 右の子の左の子に、親をつける
            right.left = mem::replace(node, None);
        }
        // 親の位置に右の子を持ってくる
        *node = right;
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

    /// 削除したいノードを葉まで持っていって削除
    fn delete(&mut self, key: u32) {
        Self::_delete(&mut self.root, key);
    }

    fn _delete(node: &mut Option<Box<Node>>, key: u32) {
        if let Some(ref mut unwraped) = node {
            if key < unwraped.key {
                Self::_delete(&mut unwraped.left, key);
            } else if key > unwraped.key {
                Self::_delete(&mut unwraped.right, key);
            } else {
                Self::__delete(node);
            }
        }
    }

    fn __delete(node: &mut Option<Box<Node>>) {
        if let Some(ref mut unwraped) = node {
            if unwraped.left.is_some() && unwraped.right.is_some() {
                if unwraped.left.as_ref().unwrap().pri > unwraped.right.as_ref().unwrap().pri {
                    Self::right_rotate(node);
                    // 右回転すると消したいノードは右の子になる
                    Self::__delete(&mut node.as_mut().unwrap().right);
                } else {
                    Self::left_rotate(node);
                    // 左回転すると消したいノードは左の子になる
                    Self::__delete(&mut node.as_mut().unwrap().left);
                }
            } else if unwraped.left.is_some() {
                Self::right_rotate(node);
                // 右回転すると消したいノードは右の子になる
                Self::__delete(&mut node.as_mut().unwrap().right);
            } else if unwraped.right.is_some() {
                Self::left_rotate(node);
                // 左回転すると消したいノードは左の子になる
                Self::__delete(&mut node.as_mut().unwrap().left);
            } else {
                *node = None;
            }
        }
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
        assert_eq!(String::from("10 20 30"), treap.inorder());
        assert_eq!(String::from("20 10 30"), treap.preorder());
    }

    #[test]
    fn test_insert_1_2() {
        let mut treap = Treap::new();
        treap.insert(20, 99);
        treap.insert(10, 50);
        treap.insert(30, 80);
        treap.delete(10);
        assert_eq!(String::from("20 30"), treap.inorder());
        assert_eq!(String::from("20 30"), treap.preorder());
    }

    #[test]
    fn test_insert_1_3() {
        let mut treap = Treap::new();
        treap.insert(20, 99);
        treap.insert(10, 50);
        treap.insert(30, 80);
        treap.delete(30);
        assert_eq!(String::from("10 20"), treap.inorder());
        assert_eq!(String::from("20 10"), treap.preorder());
    }

    #[test]
    fn test_insert_1_4() {
        let mut treap = Treap::new();
        treap.insert(20, 99);
        treap.insert(10, 50);
        treap.insert(30, 80);
        treap.insert(5, 30);
        treap.delete(10);
        assert_eq!(String::from("5 20 30"), treap.inorder());
        assert_eq!(String::from("20 5 30"), treap.preorder());
    }

    #[test]
    fn test_insert_1_5() {
        let mut treap = Treap::new();
        treap.insert(20, 99);
        treap.insert(10, 50);
        treap.insert(30, 80);
        treap.insert(40, 30);
        treap.delete(30);
        assert_eq!(String::from("10 20 40"), treap.inorder());
        assert_eq!(String::from("20 10 40"), treap.preorder());
    }

    #[test]
    fn test_insert_1_6() {
        let mut treap = Treap::new();
        treap.insert(20, 99);
        treap.insert(10, 50);
        treap.insert(30, 80);
        treap.delete(20);
        assert_eq!(String::from("10 30"), treap.inorder());
        assert_eq!(String::from("30 10"), treap.preorder());
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

    #[test]
    fn test_insert_6() {
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
        treap.delete(35);
        treap.delete(99);
        assert_eq!(String::from("6 3 1 80 14 7 21 42 86"), treap.preorder());
        assert_eq!(String::from("1 3 6 7 14 21 42 80 86"), treap.inorder());
    }
    #[test]
    fn test_insert_7() {
        let mut treap = Treap::new();
        treap.insert(53, 86);
        treap.insert(180, 1);
        treap.insert(100, 20);
        treap.insert(108, 70);
        treap.insert(20, 50);
        treap.insert(10, 25);
        treap.insert(18, 100);
        treap.insert(5, 10);
        treap.insert(13, 71);
        treap.insert(14, 8);
        treap.insert(16, 120);
        treap.insert(15, 3);
        treap.insert(12, 17);
        treap.insert(2, 2);
        assert_eq!(String::from("2 5 10 12 13 14 15 16 18 20 53 100 108 180"), treap.inorder());
        assert_eq!(String::from("16 13 10 5 2 12 14 15 18 53 20 108 100 180"), treap.preorder());
    }
    #[test]
    fn test_insert_8() {
        let mut treap = Treap::new();
        treap.insert(53, 86);
        treap.insert(180, 1);
        treap.insert(100, 20);
        treap.insert(108, 70);
        treap.insert(20, 50);
        treap.insert(10, 25);
        treap.insert(18, 100);
        treap.insert(5, 10);
        treap.insert(13, 71);
        treap.insert(14, 8);
        treap.insert(16, 120);
        treap.insert(15, 3);
        treap.insert(12, 17);
        treap.insert(2, 2);
        treap.delete(13);
        assert_eq!(String::from("2 5 10 12 14 15 16 18 20 53 100 108 180"), treap.inorder());
        assert_eq!(String::from("16 10 5 2 12 14 15 18 53 20 108 100 180"), treap.preorder());
        treap.delete(18);
        assert_eq!(String::from("2 5 10 12 14 15 16 20 53 100 108 180"), treap.inorder());
        assert_eq!(String::from("16 10 5 2 12 14 15 53 20 108 100 180"), treap.preorder());
        treap.delete(16);
        assert_eq!(String::from("2 5 10 12 14 15 20 53 100 108 180"), treap.inorder());
        assert_eq!(String::from("53 20 10 5 2 12 14 15 108 100 180"), treap.preorder());
        treap.delete(53);
        assert_eq!(String::from("2 5 10 12 14 15 20 100 108 180"), treap.inorder());
        assert_eq!(String::from("108 20 10 5 2 12 14 15 100 180"), treap.preorder());
    }
}
