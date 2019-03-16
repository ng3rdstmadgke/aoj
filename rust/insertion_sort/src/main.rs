use std::fmt::Write;

fn main() {
    let n = read::<usize>();
    let vec = read_as_vec::<u16>();
    insertion_sort(n, vec);
}

fn insertion_sort(n: usize, mut vec: Vec<u16>) -> Vec<u16> {
    for i in 0usize..n {
        let e = vec[i];
        for j in (0usize..i).rev() {
            if e < vec[j] {
                vec[j + 1] = vec[j];
                vec[j] = e;
            } else {
                break;
            }
        }
        println!("{}", join(' ', &vec));
    }
    vec
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
    fn test_insertion_sort_1() {
        let n = 6usize;
        let vec: Vec<u16> = vec![5, 2, 4, 6, 1, 3];
        assert_eq!(vec![1, 2, 3, 4, 5, 6],  insertion_sort(n, vec));
    }

    #[test]
    fn test_insertion_sort_2() {
        let n = 10usize;
        let vec: Vec<u16> = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], insertion_sort(n, vec));
    }

    #[test]
    fn test_join_1() {
        let vec: Vec<u16> = vec![1, 2, 3, 4, 5];
        assert_eq!("1 2 3 4 5".to_string(), join(' ', &vec));
    }

    #[test]
    fn test_join_2() {
        let arr: [i32; 5] = [1, 2, 3, 4, 5];
        assert_eq!("1 2 3 4 5".to_string(), join(' ', &arr));
    }
}
