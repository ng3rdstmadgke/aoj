fn main() {
    read::<u32>();
    let arr: Vec<u32> = read_as_vec();
    read::<u32>();
    let keys: Vec<u32> = read_as_vec();
    let mut result: u32 = 0;
    for key in keys {
        result += if search(&arr, key) { 1 } else { 0 };
    }
    println!("{}", result);
}

fn search(arr: &[u32], key: u32) -> bool {
    let mut s = 0;
    let mut e = arr.len();
    while s < e {
        let pivot = (s + e) / 2;
        if key < arr[pivot]  {
            e = pivot;
        } else if key > arr[pivot] {
            s = pivot + 1;
        } else {
            return true;
        }
    }
    false
}

fn read<T: std::str::FromStr>() -> T {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<T>().ok().unwrap()
}

fn read_as_vec<T: std::str::FromStr>() -> Vec<T> {
    read::<String>()
        .split_whitespace()
        .map(|e| { e.parse().ok().unwrap() })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_search_1() {
        let vec = vec![1,2,3,4,5];
        assert_eq!(true, search(&vec, 1));
        assert_eq!(true, search(&vec, 2));
        assert_eq!(true, search(&vec, 3));
        assert_eq!(true, search(&vec, 4));
        assert_eq!(true, search(&vec, 5));
    }

    #[test]
    fn test_search_2() {
        let vec = vec![2,4,6,8,10];
        assert_eq!(false, search(&vec, 1));
        assert_eq!(false, search(&vec, 3));
        assert_eq!(false, search(&vec, 5));
        assert_eq!(false, search(&vec, 7));
        assert_eq!(false, search(&vec, 9));
        assert_eq!(false, search(&vec, 11));
    }

}

