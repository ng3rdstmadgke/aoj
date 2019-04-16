fn main() {
    let input: Vec<i32> = read_as_vec();
    let w = input[0];
    let h = input[1];
    let x = input[2];
    let y = input[3];
    let r = input[4];
    if (x - r) >= 0 &&
       (y - r) >= 0 &&
       (x + r) <= w &&
       (y + r) <= h {
      println!("Yes");
    } else {
      println!("No");
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
