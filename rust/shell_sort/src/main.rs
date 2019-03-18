use std::fmt::Write;
use std::io::Write as IoWrite;

fn main() {
    let n: usize = read();
    let mut vec: Vec<u32> = (0..n).map(|_| { read() }).collect();
    let (cnt, base) = shell_sort(n, &mut vec);
    let mut stdout = std::io::stdout();
    write!(stdout, "{}\n", base.len()).unwrap();
    write!(stdout, "{}\n", join(' ', &base)).unwrap();
    write!(stdout, "{}\n", cnt).unwrap();
    write!(stdout, "{}\n", join('\n', &vec)).unwrap();
}

fn shell_sort(n: usize, vec: &mut Vec<u32>) -> (u32, Vec<usize>) {
    let group = group(n);
    let cnt = group.iter().fold(0, |cnt, step| { cnt + insertion_sort(n, vec, *step) });
    (cnt, group)
}

fn insertion_sort(n: usize, vec: &mut Vec<u32>, step: usize) -> u32 {
    let mut cnt = 0;
    for i in step..n {
        let mut j = i;
        while j >= step {
            if vec[j - step] > vec[j] {
                let tmp       = vec[j];
                vec[j]        = vec[j - step];
                vec[j - step] = tmp;
                j -= step;
                cnt += 1;
            } else {
                break;
            }
        }
    }
    cnt
}


fn group(n: usize) -> Vec<usize> {
    let mut group: Vec<usize> = vec![1];
    let mut step: usize = 1;
    loop {
        step = step * 3 + 1;
        if step < n {
            group.push(step);
        } else {
            break;
        }
    }
    group.into_iter().rev().collect()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shell_sort_1() {
        let mut v: Vec<u32> = vec![5, 1, 4, 3, 2];
        let n: usize = v.len();
        let (cnt, group) = shell_sort(n, &mut v);
        assert_eq!(vec![4, 1], group);
        assert_eq!(3, cnt);
        assert_eq!(vec![1, 2, 3, 4, 5], v);
        
    }

    #[test]
    fn test_shell_sort_2() {
        let mut v: Vec<u32> = vec![15, 12, 8, 9, 3, 2, 7, 2, 11, 1];
        let n: usize = v.len();
        let (cnt, group) = shell_sort(n, &mut v);
        assert_eq!(vec![4, 1], group);
        assert_eq!(16, cnt);
        assert_eq!(vec![1, 2, 2, 3, 7, 8, 9, 11, 12, 15], v);
    }
}
