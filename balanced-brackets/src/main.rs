use std::collections::HashMap;

fn is_balanced(s: &str) -> String {
    let brackets = HashMap::from([('(', ')'), ('{', '}'), ('[', ']')]);
    let mut stack = vec![];
    for c in s.chars() {
        if brackets.contains_key(&c) {
            if let Some(last_open) = stack.pop() {
                if brackets[&c] != last_open {
                    return String::from("NO");
                }
            } else {
                return String::from("NO");
            }
        } else {
            stack.push(c);
        }
    }
    println!("{:?}", stack);
    if stack.is_empty() {
        return String::from("YES");
    } else {
        return String::from("NO");
    }
}

fn main() {
    let s = "{{[[(())]]}}";
    let st = is_balanced(s);
    println!("{}", st);
}
