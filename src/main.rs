fn main() {
    for x in 1..10 {
        println!("Hello, world! {}", match x {
            1 => "x",
            _ => "none"
        });
    }
    let mut s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
    add_text(&mut s1);
    println!("The length of '{}' is {}.", s1, s1.len());
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn add_text(s: &mut String) {
    s.push_str(" mutated")
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        let mut s1 = String::from("hello");
        add_text(&mut s1);
        assert_eq!(s1, "hello mutated");
    }
}