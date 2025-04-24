pub fn rpn(input: &str) {
    let mut stack = Vec::new();
    let tokens = input.split_whitespace();

    for token in tokens {
        match token {
            "+" => {
                if stack.len() < 2 {
                    println!("Error");
                    return;
                }
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a + b);
            },
            "-" => {
                if stack.len() < 2 {
                    println!("Error");
                    return;
                }
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a - b);
            },
            "*" => {
                if stack.len() < 2 {
                    println!("Error");
                    return;
                }
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a * b);
            },
            "/" => {
                if stack.len() < 2 {
                    println!("Error");
                    return;
                }
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                if b == 0 {
                    println!("Error");
                    return;
                }
                stack.push(a / b);
            },
            "%" => {
                if stack.len() < 2 {
                    println!("Error");
                    return;
                }
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                if b == 0 {
                    println!("Error");
                    return;
                }
                stack.push(a % b);
            },
            _ => {
                match token.parse::<i64>() {
                    Ok(num) => stack.push(num),
                    Err(_) => {
                        println!("Error");
                        return;
                    },
                }
            }
        }
    }

    if stack.len() == 1 {
        println!("{}", stack.pop().unwrap());
    } else {
        println!("Error");
    }
}
