#[cfg(test)]

mod tests {
    extern crate binary_utils;

    use binary_utils::binary::Stream;

    #[test]
    fn test() {
        let mut stream = Stream::new(vec![], 0);

        stream.put_unsigned_var_long(1234456);
        println!("stream: {:?}", stream.get_buffer());
        let result = stream.get_unsigned_var_long();
        println!("result: {}", result);

    }
}
