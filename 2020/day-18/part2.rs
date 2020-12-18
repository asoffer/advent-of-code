#[derive(Debug, PartialEq, Eq)]
enum Entity {
    Operator(char),
    Value(i64),
    LeftParen,
    RightParen,
}

fn replace_adds(stack: &mut Vec<Entity>) {
    for i in 0..(stack.len() - 1) {
        if stack[i] == Entity::Operator('+') {
            if let Entity::Value(lhs) = stack[i - 1] {
                if let Entity::Value(rhs) = stack[i + 1] {
                    stack[i + 1] = Entity::Value(lhs + rhs);
                    stack[i] = Entity::Operator('*');
                    stack[i - 1] = Entity::Value(1);
                }
            }
        }
    }
}

fn try_reduce(stack: &mut Vec<Entity>, done: bool) -> bool {
    replace_adds(stack);
    if stack.len() >= 4 &&
        stack[stack.len() - 3] == Entity::Operator('*') &&
        stack[stack.len() - 1] == Entity::RightParen {
            if let Entity::Value(lhs) = stack[stack.len() - 2] {
                if let Entity::Value(rhs) = stack[stack.len() - 4] {
                    stack.pop();
                    stack.pop();
                    stack.pop();
                    stack.pop();
                    stack.push(Entity::Value(lhs * rhs));
                    stack.push(Entity::RightParen);
                    return true;
                }
            }
    } else if stack.len() >= 3 {
        if stack[stack.len() - 3] == Entity::LeftParen && 
            stack[stack.len() - 1] == Entity::RightParen {
            stack.pop();
            let e = stack.pop().unwrap();
            *stack.last_mut().unwrap() = e;
            return true;
        } else if done && stack[stack.len() - 2] == Entity::Operator('*') {
            if let Entity::Value(lhs) = stack[stack.len() - 1] {
                if let Entity::Value(rhs) = stack[stack.len() - 3] {
                    stack.pop();
                    stack.pop();
                    stack.pop();
                    stack.push(Entity::Value(lhs * rhs));
                    return true;
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
            ')' => {
                stack.push(Entity::RightParen);
            }
            '+' => stack.push(Entity::Operator('+')),
            '*' => stack.push(Entity::Operator('*')),
            _   => {
                let n = c as i64 - '0' as i64;
                stack.push(Entity::Value(n));
            }
        }
        while try_reduce(&mut stack, false) {}
    }
    while try_reduce(&mut stack, true) {}

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
