fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_add() {
        assert_eq!(1, 1);
    }
}
