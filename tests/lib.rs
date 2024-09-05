#[cfg(test)]

mod tests {
    extern crate binary_utils;

    use binary_utils::binary::Stream;

    #[test]
    fn test() {
        let mut stream = Stream::new(vec![0, 0, 40, 76, 111, 103, 105, 110, 32, 116, 105, 109, 101, 111, 117, 116, 32, 40, 69, 114, 114, 111, 114, 32, 73, 68, 58, 32, 100, 100, 54, 48, 45, 99, 54, 51, 100, 45, 99, 56, 53, 97, 41, 0], 0);

        let reason = stream.get_var_int();//bunda da sıkıntı var gibi?
        let skip_message = stream.get_bool();
        println!("Reason: {}", reason);
        println!("Skip Msg: {}", skip_message);
        if skip_message {
            let mut length = stream.get_unsigned_var_int();
            let message = String::from_utf8(stream.get(length).unwrap()).unwrap();
            length = stream.get_unsigned_var_int();
            let filtered_message = String::from_utf8(stream.get(length).unwrap()).unwrap();
            println!("message: {}", message);
            println!("filtered_message: {}", filtered_message);

        }

    }
}
