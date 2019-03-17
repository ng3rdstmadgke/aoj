fn main() {
    let n: usize = read();
    let result: u16 = (0..n).fold(0u16, |mut sum, _| {
        let e: u32 = read();
        if is_prime(e) {
            sum += 1;
        }
        sum
    });
    println!("{}", result);
}

fn is_prime(e: u32) -> bool {
    if e == 2 {
        return true;
    }
    if e == 0 || e == 1 || e % 2 == 0 {
        return false;
    }
    let root: u32 = (e as f64).sqrt().ceil() as u32;
    let mut i = 3;
    while i <= root {
        if e % i == 0 {
            return false;
        }
        i += 2;
    }
    true
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
    fn test_is_prime_1() {
        assert_eq!(false, is_prime(21));
        assert_eq!(false, is_prime(4));
        assert_eq!(true, is_prime(2));
        assert_eq!(true, is_prime(3));
        assert_eq!(true, is_prime(3571));
    }
}
