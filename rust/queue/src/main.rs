use std::collections::VecDeque;
fn main() {
    let input = read_as_vec::<u32>();
    let n = *input.get(0).unwrap();
    let q = *input.get(1).unwrap();
    let mut que: VecDeque<Proc> = (0..n)
        .fold(VecDeque::new(), |mut que, _| {
            let input: Vec<String> = read_as_vec();
            let name = input[0].clone(); // TODO: cloneしなくても所有権を取得できるやり方があるのか
            let time = input[1].parse::<u32>().ok().unwrap();
            que.push_back(Proc::new(name, time));
            que
        });
    let mut total = 0;
    while let Some(mut p) = que.pop_front() {
        if p.time > q {
            total += q;
            p.time = p.time - q;
            que.push_back(p);
        } else {
            total += p.time;
            println!("{} {}", p.name, total);
        }
    }
}


struct Proc {
    name: String,
    time: u32,
}

impl Proc {
    fn new(name: String, time: u32) -> Self {
        Proc { name, time }
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
        .map(|e| { e.parse::<T>().ok().unwrap() })
        .collect()
}
