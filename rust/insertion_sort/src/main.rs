fn main() {
    let n = read::<usize>();
    let mut vec = read_as_vec::<u32>();
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
        println!("{}", vec.iter().map(|e| e.to_string()).collect::<Vec<String>>().join(" "));
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

