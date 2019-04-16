use std::fmt::Write;

fn main() {
    let n: usize = read();
    let mut graph: Vec<Vec<i32>> = vec![vec![0;n + 1]; n + 1];
    for _ in 0..n {
        let input: Vec<i32> = read_as_vec();
        let v: usize = input[0] as usize;
        let slice: &[i32] = &input[2..];
        for &i in slice {
            graph[v][i as usize] = 1;
        }
    }

    for vec in &graph[1..] {
        println!("{}", join(' ', &vec[1..]));
    }
    //println!("{:?}", graph);
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
