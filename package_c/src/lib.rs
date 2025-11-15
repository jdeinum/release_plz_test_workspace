pub fn add_4(left: u64, right: u64) -> u64 {
    left + right + 4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_4(2, 2);
        assert_eq!(result, 8);
    }
}
