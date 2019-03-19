fn main() {
    let input = read::<String>();
    let expr: Vec<&str> = input.split_whitespace().collect();
    let result: i32 = rpn(&expr);
    println!("{}", result);
}

fn rpn(expr: &[&str]) -> i32 {
    let mut stack: Vec<i32> = Vec::new();
    for e in expr {
        match e.parse::<i32>() {
            Ok(n) => {
                stack.push(n);
            },
            Err(_) => {
                let right = stack.pop().unwrap();
                let left  = stack.pop().unwrap();
                if *e == "+" {
                    stack.push(left + right);
                } else if *e == "-" {
                    stack.push(left - right);
                } else if *e == "*" {
                    stack.push(left * right);
                } else if *e == "/" {
                    stack.push(left / right);
                }
            },
        }
    }
    stack[0]
}

fn read<T: std::str::FromStr>() -> T {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<T>().ok().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test_rpn_1() {
        let input = String::from("34 116 + 20 5 - 5 - 1 * +");
        let expr: Vec<&str> = input.split_whitespace().collect();
        assert_eq!(160i32, rpn(&expr));
    }

    #[test]
    fn test_rpn_2() {
        let input = String::from("500 132 + 132 - 2 * 100 10 10 * + - 50 50 50 50 + + + + 10 999 * + 24 +");
        let expr: Vec<&str> = input.split_whitespace().collect();
        assert_eq!(11014i32, rpn(&expr));
    }
}
