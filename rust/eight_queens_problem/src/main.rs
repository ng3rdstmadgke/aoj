use std::fmt::Write;

fn main() {
    let n: i32 = 13;
    let mut queens: Vec<i32> = vec![0; (n as usize)];
    let mut ret: Vec<String> = vec![];
    n_queen(n, 0, 0, &mut queens, &mut ret);
    println!("{}", ret.len());
}

fn n_queen(n: i32, y: i32, x: i32, queens: &mut [i32], ret: &mut Vec<String>) {
    if y < n {
        if can_put_queen(y, x, queens) {
            queens[(y as usize)] = x;
            n_queen(n, y + 1, 0, queens, ret);
        }
        if (x + 1) < n {
            n_queen(n, y, x + 1, queens, ret);
        }
    } else {
        ret.push(join(' ', queens));
    }
}

fn can_put_queen(y: i32, x: i32, queens: &[i32]) -> bool {
    for (i, &e) in queens.iter().enumerate() {
        let i = i as i32;
        if i == y {
            return true;
        }
        if e == x || e + (y - i) == x || e - (y - i) == x {
            return false;
        }
    }
    true
}


fn join<T: std::fmt::Display>(delimiter: char, arr: &[T]) -> String {
    let mut text = String::new();
    for (i, e) in arr.iter().enumerate() {
        if i > 0 {
            text.push(delimiter);
        }
        write!(text, "{}", e).unwrap();
    }
    text
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
    fn test_can_put_queen() {
        let queens = vec![0, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(true, can_put_queen(0, 0, &queens));
        let queens = vec![1, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(false, can_put_queen(1, 0, &queens));
        assert_eq!(false, can_put_queen(1, 1, &queens));
        assert_eq!(false, can_put_queen(1, 2, &queens));
        let queens = vec![3, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(false, can_put_queen(2, 1, &queens));
        assert_eq!(false, can_put_queen(2, 3, &queens));
        assert_eq!(false, can_put_queen(2, 5, &queens));
        let queens = vec![3, 6, 2, 7, 1, 4, 0, 0];
        assert_eq!(false, can_put_queen(7, 1, &queens));
        assert_eq!(false, can_put_queen(7, 2, &queens));
        assert_eq!(false, can_put_queen(7, 3, &queens));
        assert_eq!(false, can_put_queen(7, 4, &queens));
        assert_eq!(true , can_put_queen(7, 5, &queens));
        assert_eq!(false, can_put_queen(7, 6, &queens));
        assert_eq!(false, can_put_queen(7, 7, &queens));
    }

    #[test]
    fn test_n_queen() {
        let n: i32 = 8;
        let mut queens: Vec<i32> = vec![0; (n as usize)];
        let mut ret: Vec<String> = vec![];
        n_queen(n, 0, 0, &mut queens, &mut ret);
        assert_eq!(92, ret.len());

        let n: i32 = 9;
        let mut queens: Vec<i32> = vec![0; (n as usize)];
        let mut ret: Vec<String> = vec![];
        n_queen(n, 0, 0, &mut queens, &mut ret);
        assert_eq!(352, ret.len());

        let n: i32 = 10;
        let mut queens: Vec<i32> = vec![0; (n as usize)];
        let mut ret: Vec<String> = vec![];
        n_queen(n, 0, 0, &mut queens, &mut ret);
        assert_eq!(724, ret.len());
    }
}
