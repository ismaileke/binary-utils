#[cfg(test)]

mod tests {
    extern crate binary_utils;

    use binary_utils::binary::Stream;

    // ===== 24-bit Integer Tests =====

    #[test]
    fn test_u24_be() {
        let mut stream = Stream::with_capacity(10);
        stream.put_u24_be(0xFFFFFF);
        stream.rewind();
        assert_eq!(stream.get_u24_be(), 0xFFFFFF);
    }

    #[test]
    fn test_u24_le() {
        let mut stream = Stream::with_capacity(10);
        stream.put_u24_le(0xFFFFFF);
        stream.rewind();
        assert_eq!(stream.get_u24_le(), 0xFFFFFF);
    }

    // ===== VarInt Tests =====

    #[test]
    fn test_var_u32_small() {
        let mut stream = Stream::with_capacity(10);
        stream.put_var_u32(0);
        stream.rewind();
        assert_eq!(stream.get_var_u32(), 0);
    }

    #[test]
    fn test_var_u32_large() {
        let mut stream = Stream::with_capacity(10);
        stream.put_var_u32(0xFFFFFFFF);
        stream.rewind();
        assert_eq!(stream.get_var_u32(), 0xFFFFFFFF);
    }

    #[test]
    fn test_var_i32_positive() {
        let mut stream = Stream::with_capacity(10);
        stream.put_var_i32(100);
        stream.rewind();
        assert_eq!(stream.get_var_i32(), 100);
    }

    #[test]
    fn test_var_i32_negative() {
        let mut stream = Stream::with_capacity(10);
        stream.put_var_i32(-100);
        stream.rewind();
        let result = stream.get_var_i32();
        println!("var_i32(-100) = {}", result);
        assert_eq!(result, -100);
    }

    // ===== VarLong Tests =====

    #[test]
    fn test_var_u64_large() {
        let mut stream = Stream::with_capacity(20);
        stream.put_var_u64(0xFFFFFFFFFFFFFFFF);
        stream.rewind();
        assert_eq!(stream.get_var_u64(), 0xFFFFFFFFFFFFFFFF);
    }

    #[test]
    fn test_var_i64_negative() {
        let mut stream = Stream::with_capacity(20);
        stream.put_var_i64(-1);
        stream.rewind();
        let result = stream.get_var_i64();
        println!("var_i64(-1) = {}", result);
        assert_eq!(result, -1);
    }

    #[test]
    fn test_var_i64_positive() {
        let mut stream = Stream::with_capacity(20);
        stream.put_var_i64(1000);
        stream.rewind();
        assert_eq!(stream.get_var_i64(), 1000);
    }
}
