pub fn add_3(left: u64, right: u64) -> u64 {
    left + right + 3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_3(2, 2);
        assert_eq!(result, 7);
    }
}
