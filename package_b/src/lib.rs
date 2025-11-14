pub fn add_5(left: u64, right: u64) -> u64 {
    left + right + 5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_5(2, 2);
        assert_eq!(result, 9);
    }
}
