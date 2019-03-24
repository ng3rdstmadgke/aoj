use std::fmt::Write;

fn main() {
    let n: usize = read();
    let mut vec: Vec<i32> = read_as_vec();
    build_max_heap(n, &mut vec);
    println!(" {}", join(' ', &vec));
}

fn build_max_heap(len: usize, slice: &mut [i32]) {
    for i in (0..(len / 2)).rev() {
        max_heapify(len, slice, i);
    }
}

fn max_heapify(len: usize, slice: &mut [i32], idx: usize) {
    let left  = idx * 2 + 1;
    let right = idx * 2 + 2;
    let me = slice[idx];
    if right < len {      // 子ノードが2つ
        let max = if slice[left] > slice[right] { left } else { right };
        if slice[max] > me {
            slice[idx] = slice[max];
            slice[max] = me;
            max_heapify(len, slice, max);
        }
    } else if left < len { // 子ノードが1つ
        if slice[left] > me {
            slice[idx] = slice[left];
            slice[left] = me;
            max_heapify(len, slice, left);
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

fn join<T: std::fmt::Display>(delimiter: char, arr: &[T]) -> String {
    let mut text = String::new();
    for e in arr.iter() {
        write!(text, "{}", e).unwrap();
        text.push(delimiter);
    }
    text.pop();
    text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_max_heap_1() {
        let len: usize = 10;
        let mut vec: Vec<i32> = vec![4, 1, 3, 2, 16, 9, 10, 14, 8, 7];
        build_max_heap(len, &mut vec);
        assert_eq!(vec![16, 14, 10, 8, 7, 9, 3, 2, 4, 1], vec);
    }

    #[test]
    fn test_build_max_heap_2() {
        let len: usize = 16;
        let mut vec: Vec<i32> = vec![-1000000000, 88, 6, 3, 100, 4, 5, 7, 15, 21, 50, 999999999, 53, 23, 8, 18];
        build_max_heap(len, &mut vec);
        assert_eq!(vec![999999999, 100, 53, 18, 88, 6, 23, 7, 15, 21, 50, 4, -1000000000, 5, 8, 3], vec);

    }
}


