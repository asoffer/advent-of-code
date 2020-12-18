#[derive(Debug, PartialEq, Eq)]
enum Entity {
    Operator(char),
    Value(i64),
    LeftParen,
    RightParen,
}

fn try_reduce(stack: &mut Vec<Entity>) -> bool {
    if stack.len() >= 3 {
        if stack[stack.len() - 3] == Entity::LeftParen && 
           stack[stack.len() - 1] == Entity::RightParen {
            stack.pop();
            let e = stack.pop().unwrap();
            *stack.last_mut().unwrap() = e;
            return true;
        } else if let Entity::Value(lhs) = stack[stack.len() - 3] {
            if let Entity::Value(rhs) = stack[stack.len() - 1] {
                return match stack[stack.len() - 2] {
                    Entity::Operator('+') => {
                        stack.pop();
                        stack.pop();
                        stack.pop();
                        stack.push(Entity::Value(lhs + rhs));
                        true
                    },
                    Entity::Operator('*') => {
                        stack.pop();
                        stack.pop();
                        stack.pop();
                        stack.push(Entity::Value(lhs * rhs));
                        true
                    }
                    _ => false,
                }
            }
        }
    }
    return false;
}

fn evaluate(line: &str) -> i64 {
    let mut stack = Vec::<Entity>::new();
    for c in line.chars() {
        match c {
            ' ' => {},
            '(' => stack.push(Entity::LeftParen),
            ')' => stack.push(Entity::RightParen),
            '+' => stack.push(Entity::Operator('+')),
            '*' => stack.push(Entity::Operator('*')),
            _   => {
                let n = c as i64 - '0' as i64;
                stack.push(Entity::Value(n));
            }
        }

        while try_reduce(&mut stack) {}
    }
    while try_reduce(&mut stack) {}

    let total: i64 = match stack[0] {
        Entity::Value(n) => n,
        _ => panic!(),
    };
    total
}

fn main() {
    println!("{}", std::fs::read_to_string("input.txt").expect("failed to read file")
        .lines()
        .fold(0, |acc, line| acc + evaluate(line)));
}
