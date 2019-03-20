use std::fmt::Write;

fn main() {
    let _n: usize = read();
    let mut vec: Vec<u32> = read_as_vec();
    let pivot_idx = partition(&mut vec);
    println!("{}", join_for_partition(' ', &vec, pivot_idx));
}

fn quick_sort(mut slice: &mut [u32]) {
    let n = slice.len();
    if n > 1 {
        let pivot_idx = partition(&mut slice);
        let (s1, s2) = slice.split_at_mut(pivot_idx);
        quick_sort(s1);
        quick_sort(s2);
    }
}

fn partition(slice: &mut [u32]) -> usize {
    let n = slice.len();
    let pivot = slice[n - 1];
    let mut pre_ptr  = 0usize;
    let mut post_ptr = 0usize;
    while pre_ptr < (n - 1) {
        if slice[pre_ptr] <= pivot {
            let tmp = slice[pre_ptr];
            slice[pre_ptr]  = slice[post_ptr];
            slice[post_ptr] = tmp;
            post_ptr += 1;
        }
        pre_ptr += 1;
    }
    slice[pre_ptr]  = slice[post_ptr];
    slice[post_ptr] = pivot;
    post_ptr
}

fn join_for_partition<T: std::fmt::Display>(delimiter: char, arr: &[T], idx: usize) -> String {
    let mut text = String::new();
    for (i, e) in arr.iter().enumerate() {
        if i > 0 {
            text.push(delimiter);
        }
        if i == idx {
            write!(text, "[{}]", e).unwrap();
        } else {
            write!(text, "{}", e).unwrap();
        }
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

fn _join<T: std::fmt::Display>(delimiter: char, arr: &[T]) -> String {
    let mut text = String::new();
    for (i, e) in arr.iter().enumerate() {
        if i > 0 {
            text.push(delimiter);
        }
        write!(text, "{}", e).unwrap();
    }
    text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partition_1() {
        let mut vec: Vec<u32> = vec![13, 19, 9, 5, 12, 8, 7, 4, 21, 2, 6, 11];
        let pivot_idx = partition(&mut vec);
        assert_eq!(vec![9, 5, 8, 7, 4, 2, 6, 11, 21, 13, 19, 12], vec);
        assert_eq!(7, pivot_idx);
    }

    #[test]
    fn test_partition_2() {
        let mut vec: Vec<u32> = vec![3, 1, 1, 2, 3, 4, 2];
        let pivot_idx = partition(&mut vec);
        assert_eq!(vec![1, 1, 2, 2, 3, 4, 3], vec);
        assert_eq!(3, pivot_idx);
    }

    #[test]
    fn test_partition_3() {
        let mut vec: Vec<u32> = vec![3];
        let pivot_idx = partition(&mut vec);
        assert_eq!(vec![3], vec);
        assert_eq!(0, pivot_idx);
    }

    #[test]
    fn test_partition_4() {
        let mut vec: Vec<u32> = vec![3, 2];
        let pivot_idx = partition(&mut vec);
        assert_eq!(vec![2, 3], vec);
        assert_eq!(0, pivot_idx);
    }

    #[test]
    fn test_partition_5() {
        let mut vec: Vec<u32> = vec![2, 3];
        let pivot_idx = partition(&mut vec);
        assert_eq!(vec![2, 3], vec);
        assert_eq!(1, pivot_idx);
    }

    #[test]
    fn test_quick_sort_1() {
        let mut vec: Vec<u32> = vec![13, 19, 9, 5, 12, 8, 7, 4, 21, 2, 6, 11];
        let pivot_idx = quick_sort(&mut vec);
        assert_eq!(vec![2, 4, 5, 6, 7, 8, 9, 11, 12, 13, 19, 21], vec);
    }

    #[test]
    fn test_quick_sort_2() {
        let mut vec: Vec<u32> = vec![3, 1, 1, 2, 3, 4, 2];
        let pivot_idx = quick_sort(&mut vec);
        assert_eq!(vec![1, 1, 2, 2, 3, 3, 4], vec);
    }

}
