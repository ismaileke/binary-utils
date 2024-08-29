#[cfg(test)]

mod tests {
    extern crate binary_utils;

    use binary_utils::binary::Stream;

    #[test]
    fn test() {
        let mut stream = Stream::new(vec![], 0);
        stream.put_unsigned_var_int(2322211);
        println!("{:?}", stream.get_buffer());
        println!("{}", stream.get_unsigned_var_int());

    }
}
