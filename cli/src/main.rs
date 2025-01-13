fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn is_true() {
        assert_eq!(true, true)
    }
}
