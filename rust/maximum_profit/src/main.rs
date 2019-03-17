fn main() {
    let n: u32 = read();
    let reader: Box<Iterator<Item = i32>> = Box::new(ReadLines::new(n, 0));
    println!("{}", maximum_profit(reader));
}

fn maximum_profit(reader: Box<Iterator<Item = i32>>) -> i32 {
    let mut min    = i32::max_value();
    let mut profit = i32::min_value();
    for e in reader {
        let sub = e - min;
        if  sub > profit {
            profit = sub;
        }
        if e < min {
            min = e;
        }
    }
    profit
}

fn read<T: std::str::FromStr>() -> T {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<T>().ok().unwrap()
}

struct ReadLines<T: std::str::FromStr> {
    n: u32,
    cnt: u32,
    _d: T,
}
impl<T: std::str::FromStr> ReadLines<T> {
    fn new(n: u32, d: T) -> Self { ReadLines { n: n, cnt: 0, _d: d } }
}

impl<T: std::str::FromStr> Iterator for ReadLines<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        let ret = if self.cnt < self.n {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            Some(input.trim().parse::<T>().ok().unwrap())
        } else {
            None
        };
        self.cnt += 1;
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_profit_1() {
        let data: Vec<i32> = vec![5, 3, 1, 3, 4, 3];
        let d: Box<Iterator<Item = i32>> = Box::new(data.into_iter());
        assert_eq!(3, maximum_profit(d));
    }

    #[test]
    fn test_maximum_profit_2() {
        let data: Vec<i32> = vec![4, 3, 2];
        let d: Box<Iterator<Item = i32>> = Box::new(data.into_iter());
        assert_eq!(-1, maximum_profit(d));
    }
}
