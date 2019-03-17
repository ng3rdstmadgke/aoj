use std::fmt::Write;

fn main() {
    let n: usize = read();
    let mut vec: Vec<u8> = read_as_vec();
    let cnt = bubble_sort(n, &mut vec);
    println!("{}", join(' ', &vec));
    println!("{}", cnt);
}

fn bubble_sort(n: usize, vec: &mut Vec<u8>) -> u32 {
    let mut cnt = 0u32;
    let mut flag = true;
    while flag {
        flag = false;
        for i in (1..n).rev() {
            if vec[i - 1] > vec[i] {
                let tmp    = vec[i];
                vec[i]     = vec[i - 1];
                vec[i - 1] = tmp;
                cnt += 1;
                flag = true;
            }
        }
    }
    cnt
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
    fn test_bubble_sort_1() {
        let mut vec = vec![5, 3, 2, 4, 1];
        let cnt = bubble_sort(5, &mut vec);
        assert_eq!(vec![1, 2, 3, 4, 5], vec);
        assert_eq!(8, cnt);
    }

    #[test]
    fn test_bubble_sort_2() {
        let mut vec = vec![5, 2, 4, 6, 1, 3];
        let cnt = bubble_sort(6, &mut vec);
        assert_eq!(vec![1, 2, 3, 4, 5, 6], vec);
        assert_eq!(9, cnt);
    }
}
