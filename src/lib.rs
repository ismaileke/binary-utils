pub mod binary {
    pub struct Stream {
        buffer: Vec<u8>,
        offset: u32,
    }

    impl Stream {
        #[inline]
        pub fn new(buffer: Vec<u8>, offset: u32) -> Self {
            Self { buffer, offset }
        }

        #[inline]
        pub fn with_capacity(capacity: usize) -> Self {
            Self {
                buffer: Vec::with_capacity(capacity),
                offset: 0,
            }
        }

        #[inline]
        pub fn rewind(&mut self) {
            self.offset = 0;
        }

        #[inline]
        pub fn set_offset(&mut self, offset: u32) {
            if offset <= self.buffer.len() as u32 {
                self.offset = offset;
            } else {
                panic!("Offset out of bounds");
            }
        }

        #[inline]
        pub fn get_offset(&self) -> u32 {
            self.offset
        }

        #[inline]
        pub fn get_buffer(&self) -> &[u8] {
            &self.buffer
        }

        pub fn get(&mut self, length: u32) -> Vec<u8> {
            let end_index: usize = (self.offset + length) as usize;

            if end_index <= self.buffer.len() {
                let start_index: usize = self.offset as usize;
                self.offset += length;
                self.buffer[start_index..end_index].to_vec()
            } else {
                panic!("The specified range is invalid.");
            }
        }

        pub fn get_remaining(&self) -> Vec<u8> {
            if self.offset >= self.buffer.len() as u32 {
                panic!("Error get_remaining(): No bytes left to read");
            } else {
                self.buffer[self.offset..].to_vec()
            }
        }

        #[inline]
        pub fn put(&mut self, value: Vec<u8>) {
            self.buffer.extend(value);
        }

        #[inline]
        pub fn get_bool(&mut self) -> bool {
            let bytes = self.get(1);
            bytes[0] != 0
        }

        #[inline]
        pub fn put_bool(&mut self, value: bool) {
            self.buffer.push(if value { 0x01 } else { 0x00 });
        }

        #[inline]
        pub fn get_byte(&mut self) -> u8 {
            let bytes = self.get(1);
            bytes[0]
        }

        #[inline]
        pub fn put_byte(&mut self, value: u8) {
            self.buffer.push(value);
        }

        // Big-endian (network byte order)
        #[inline]
        pub fn get_be_unsigned_short(&mut self) -> u16 {
            let bytes = self.get(2);
            u16::from_be_bytes([bytes[0], bytes[1]])
        }

        #[inline]
        pub fn get_be_signed_short(&mut self) -> i16 {
            let bytes = self.get(2);
            i16::from_be_bytes([bytes[0], bytes[1]])
        }

        #[inline]
        pub fn put_be_unsigned_short(&mut self, value: u16) {
            self.buffer.extend_from_slice(&value.to_be_bytes());
        }

        #[inline]
        pub fn put_be_signed_short(&mut self, value: i16) {
            self.buffer.extend_from_slice(&value.to_be_bytes());
        }

        // Little-endian
        #[inline]
        pub fn get_le_unsigned_short(&mut self) -> u16 {
            let bytes = self.get(2);
            u16::from_le_bytes([bytes[0], bytes[1]])
        }

        #[inline]
        pub fn get_le_signed_short(&mut self) -> i16 {
            let bytes = self.get(2);
            i16::from_le_bytes([bytes[0], bytes[1]])
        }

        #[inline]
        pub fn put_le_unsigned_short(&mut self, value: u16) {
            self.buffer.extend_from_slice(&value.to_le_bytes());
        }

        #[inline]
        pub fn put_le_signed_short(&mut self, value: i16) {
            self.buffer.extend_from_slice(&value.to_le_bytes());
        }

        // 24-bit integers (Triad) - Big-endian
        #[inline]
        pub fn get_be_triad(&mut self) -> i32 {
            let bytes = self.get(3);
            i32::from_be_bytes([0, bytes[0], bytes[1], bytes[2]])
        }

        #[inline]
        pub fn put_be_triad(&mut self, value: i32) {
            let bytes = value.to_be_bytes();
            self.buffer.extend_from_slice(&bytes[1..4]);
        }

        // 24-bit integers (Triad) - Little-endian
        #[inline]
        pub fn get_le_triad(&mut self) -> i32 {
            let bytes = self.get(3);
            i32::from_le_bytes([bytes[0], bytes[1], bytes[2], 0])
        }

        #[inline]
        pub fn put_le_triad(&mut self, value: i32) {
            let bytes = value.to_le_bytes();
            self.buffer.extend_from_slice(&bytes[0..3]);
        }

        // 32-bit integers - Big-endian
        #[inline]
        pub fn get_be_unsigned_int(&mut self) -> u32 {
            let bytes = self.get(4);
            u32::from_be_bytes(bytes.try_into().unwrap())
        }

        #[inline]
        pub fn put_be_unsigned_int(&mut self, value: u32) {
            self.buffer.extend_from_slice(&value.to_be_bytes());
        }

        // 32-bit integers - Little-endian
        #[inline]
        pub fn get_le_unsigned_int(&mut self) -> u32 {
            let bytes = self.get(4);
            u32::from_le_bytes(bytes.try_into().unwrap())
        }

        #[inline]
        pub fn put_le_unsigned_int(&mut self, value: u32) {
            self.buffer.extend_from_slice(&value.to_le_bytes());
        }

        // 32-bit floats - Big-endian
        #[inline]
        pub fn get_be_float(&mut self) -> f32 {
            let bytes = self.get(4);
            f32::from_be_bytes(bytes.try_into().unwrap())
        }

        #[inline]
        pub fn put_be_float(&mut self, value: f32) {
            self.buffer.extend_from_slice(&value.to_be_bytes());
        }

        // 32-bit floats - Little-endian
        #[inline]
        pub fn get_le_float(&mut self) -> f32 {
            let bytes = self.get(4);
            f32::from_le_bytes(bytes.try_into().unwrap())
        }

        #[inline]
        pub fn put_le_float(&mut self, value: f32) {
            self.buffer.extend_from_slice(&value.to_le_bytes());
        }

        // 64-bit floats - Big-endian
        #[inline]
        pub fn get_be_double(&mut self) -> f64 {
            let bytes = self.get(8);
            f64::from_be_bytes(bytes.try_into().unwrap())
        }

        #[inline]
        pub fn put_be_double(&mut self, value: f64) {
            self.buffer.extend_from_slice(&value.to_be_bytes());
        }

        // 64-bit floats - Little-endian
        #[inline]
        pub fn get_le_double(&mut self) -> f64 {
            let bytes = self.get(8);
            f64::from_le_bytes(bytes.try_into().unwrap())
        }

        #[inline]
        pub fn put_le_double(&mut self, value: f64) {
            self.buffer.extend_from_slice(&value.to_le_bytes());
        }

        // 64-bit integers - Big-endian
        #[inline]
        pub fn get_be_signed_long(&mut self) -> i64 {
            let bytes = self.get(8);
            i64::from_be_bytes(bytes.try_into().unwrap())
        }

        #[inline]
        pub fn put_be_signed_long(&mut self, value: i64) {
            self.buffer.extend_from_slice(&value.to_be_bytes());
        }

        // 64-bit integers - Little-endian
        #[inline]
        pub fn get_le_signed_long(&mut self) -> i64 {
            let bytes = self.get(8);
            i64::from_le_bytes(bytes.try_into().unwrap())
        }

        #[inline]
        pub fn put_le_signed_long(&mut self, value: i64) {
            self.buffer.extend_from_slice(&value.to_le_bytes());
        }

        // Variable-length integers
        pub fn get_unsigned_var_int(&mut self) -> u32 {
            let mut value = 0u32;
            for i in 0..5 {
                let b = self.get_byte();
                value |= ((b & 0x7f) as u32) << (i * 7);
                if b & 0x80 == 0 {
                    return value;
                }
            }
            panic!("VarInt did not terminate after 5 bytes!");
        }

        pub fn put_unsigned_var_int(&mut self, mut value: u32) {
            loop {
                if value >= 0x80 {
                    self.buffer.push((value as u8) | 0x80);
                    value >>= 7;
                } else {
                    self.buffer.push(value as u8);
                    break;
                }
            }
        }

        #[inline]
        pub fn get_var_int(&mut self) -> i32 {
            let raw = self.get_unsigned_var_int();
            ((raw >> 1) as i32) ^ -((raw & 1) as i32)
        }

        #[inline]
        pub fn put_var_int(&mut self, value: i32) {
            let encoded = ((value << 1) ^ (value >> 31)) as u32;
            self.put_unsigned_var_int(encoded);
        }

        pub fn get_unsigned_var_long(&mut self) -> u64 {
            let mut value = 0u64;
            for i in 0..10 {
                let b = self.get_byte();
                value |= ((b & 0x7f) as u64) << (i * 7);
                if b & 0x80 == 0 {
                    return value;
                }
            }
            panic!("VarLong did not terminate after 10 bytes!");
        }

        pub fn put_unsigned_var_long(&mut self, mut value: u64) {
            loop {
                if value >= 0x80 {
                    self.buffer.push((value as u8) | 0x80);
                    value >>= 7;
                } else {
                    self.buffer.push(value as u8);
                    break;
                }
            }
        }

        #[inline]
        pub fn get_var_long(&mut self) -> i64 {
            let raw = self.get_unsigned_var_long();
            ((raw >> 1) as i64) ^ (-((raw & 1) as i64))
        }

        #[inline]
        pub fn put_var_long(&mut self, value: i64) {
            let encoded = ((value << 1) ^ (value >> 63)) as u64;
            self.put_unsigned_var_long(encoded);
        }

        #[inline]
        pub fn feof(&self) -> bool {
            self.offset >= self.buffer.len() as u32
        }
    }
}
