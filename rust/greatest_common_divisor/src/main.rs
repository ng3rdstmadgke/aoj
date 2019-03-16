fn main() {
    let inputs: Vec<u32> = read_as_vec();
    /*
    if let &[x, y] = &inputs[..] {
        println!("{}", gcd(x, y));
    }
    */
    println!("{}", gcd(inputs[0], inputs[1]));
}

fn gcd(x: u32, y: u32) -> u32 {
    fn sub(x: u32, y: u32) -> u32 {
        let m = x % y;
        if m == 0 {
            y
        } else {
            gcd(y, m)
        }
    }
    if x > y { sub(x, y) } else { sub(y, x) }
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
    fn test_gcd_1() {
        assert_eq!(6, gcd(923490024, 825000390));
        assert_eq!(6, gcd(825000390, 923490024));
    }
}
