#[cfg(test)]

mod tests {
    extern crate binary_utils;

    use binary_utils::binary::Stream;

    #[test]
    fn test() {
        let mut stream = Stream::new(vec![0, 52, 255], 0);
        println!("{:?}", stream.get(1));

    }
}
