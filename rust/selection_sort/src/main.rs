use std::fmt::Write;
fn main() {
    let n: usize = read();
    let mut vec: Vec<u8> = read_as_vec();
    let cnt = selection_sort(n, &mut vec);
    println!("{}", join(' ', &vec));
    println!("{}", cnt);
}

fn selection_sort(n: usize, vec: &mut Vec<u8>) -> u32 {
    let mut cnt = 0;
    for i in 0..n {
        let mut min = i;
        for j in i..n {
            if vec[j] < vec[min] {
                min = j
            }
        }
        if min != i {
            // swap
            let tmp  = vec[i];
            vec[i]   = vec[min];
            vec[min] = tmp;
            cnt += 1;
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
    fn test_selection_sort_1() {
        let mut vec: Vec<u8> = vec![5, 6, 4, 2, 1, 3];
        let n = vec.len();
        let cnt = selection_sort(n, &mut vec);
        assert_eq!(vec![1, 2, 3, 4, 5, 6], vec);
        assert_eq!(4, cnt);
    }

    #[test]
    fn test_selection_sort_2() {
        let mut vec: Vec<u8> = vec![5, 2, 4, 6, 1, 3];
        let n = vec.len();
        let cnt = selection_sort(n, &mut vec);
        assert_eq!(vec![1, 2, 3, 4, 5, 6], vec);
        assert_eq!(3, cnt);
    }
}
