use std::fmt::Write;
use std::slice;

fn main() {
    let _n: usize = read();
    let mut vec: Vec<u32> = read_as_vec();
    let cnt = merge_sort(&mut vec);
    println!("{}", join(' ', &vec));
    println!("{}", cnt);
}

fn merge_sort(slice: &mut [u32]) -> u64 {
    let mut cnt = 0u64;
    if slice.len() > 1 {
        let m = (0 + slice.len()) / 2;
        let (s0, s1, s2) = split_at_mut(slice, m);
        cnt += merge_sort(s1);
        cnt += merge_sort(s2);
        // merge
        let mut tmp = vec![0; m];
        tmp.copy_from_slice(s1);
        let mut iter1 = tmp.iter();
        let mut iter2 = s2.iter();
        let mut o1 = iter1.next();
        let mut o2 = iter2.next();
        let mut idx = 0;
        loop {
            if o1 != None && o2 != None {
                cnt += 1;
                let e1: u32 = *o1.unwrap();
                let e2: u32 = *o2.unwrap();
                if e1 <= e2 {
                    s0[idx] = e1;
                    o1 = iter1.next();
                } else {
                    s0[idx] = e2;
                    o2 = iter2.next();
                }
            } else if o1 != None {
                cnt += 1;
                s0[idx] = *o1.unwrap();
                o1 = iter1.next();
            } else if o2 != None { // この分岐は本当はいらない。
                o2 = iter2.next();
                cnt += 1;
            } else {
                break;
            }
            idx += 1;
        }
    }
    cnt
}

fn split_at_mut(slice: &mut [u32], mid: usize) -> (&mut [u32], &mut [u32], &mut [u32]) {
    let n = slice.len();
    let ptr: *mut u32 = slice.as_mut_ptr();
    assert!(mid <= n);
    unsafe {
        let s0: &mut [u32] = slice::from_raw_parts_mut(ptr, n);
        let s1: &mut [u32] = slice::from_raw_parts_mut(ptr, mid);
        let s2: &mut [u32] = slice::from_raw_parts_mut(ptr.offset(mid as isize), n - mid);
        (s0, s1, s2)
    }
}

fn join<T: std::fmt::Display>(delimiter: char, es: &Vec<T>) -> String {
    let mut output = String::new();
    for (i, e) in es.iter().enumerate() {
        if i != 0 {
            output.push(delimiter);
        }
        write!(output, "{}", e).unwrap();
    }
    output
}

fn read<T: std::str::FromStr>() -> T {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<T>().ok().unwrap()
}

fn read_as_vec<T: std::str::FromStr>() -> Vec<T> {
    read::<String>()
        .split_whitespace()
        .map(|e| { e.trim().parse::<T>().ok().unwrap() })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_sort_1() {
        let mut vec: Vec<u32> = vec![8, 5, 9, 2, 6, 3, 7, 1, 10, 4];
        assert_eq!(34u64, merge_sort(&mut vec));
        assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], vec);
    }

    #[test]
    fn test_merge_sort_2() {
        let mut vec: Vec<u32> = vec![0, 3, 3, 2, 9, 0, 8, 2, 6, 6];
        assert_eq!(34u64, merge_sort(&mut vec));
        assert_eq!(vec![0, 0, 2, 2, 3, 3, 6, 6, 8, 9], vec);
    }
}
