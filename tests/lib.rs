#[cfg(test)]

mod tests {
    extern crate binary_utils;

    use binary_utils::binary::Stream;

    #[test]
    fn test() {
        println!("{:?}", "167".to_string().into_bytes());

        let mut stream = Stream::new(vec![49, 54, 55], 0);

        let result = stream.get_var_int();
        println!("result: {}", result);

    }
}
