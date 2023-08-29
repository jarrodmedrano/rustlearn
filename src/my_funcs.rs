pub fn add_five(num: u32) -> u32 {
    5 + num
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn adds_five_test() {
        let result = add_five(5);
        assert_eq!(result, 10);
    }
}
