pub mod binary {
    pub struct Stream {
        buffer: Vec<u8>,
        offset: u16,
    }

    impl Stream {
        pub fn new(buffer: Vec<u8>, offset: u16) -> Self {
            Self {buffer, offset}
        }

        pub fn rewind(&mut self) {
            self.offset = 0;
        }

        pub fn set_offset(&mut self, offset: u16) {
            self.offset = offset
        }

        pub fn get_offset(&self) -> u16 {
            self.offset
        }

        pub fn get_buffer(&self) -> Vec<u8> {
            self.buffer.to_vec()
        }

        pub fn get(&mut self, length: u16) -> Result<Vec<u8>, String> {
            let end_index: usize = (self.offset + length) as usize;

            if end_index <= self.buffer.len() {
                let start_index: usize = self.offset as usize;
                self.offset += length;
                Ok(self.buffer[start_index..end_index].to_vec())
            } else {
                Err(String::from("The specified range is invalid."))
            }
        }


        pub fn get_remaining(&self) -> Result<Vec<u8>, String> {
            let buff_len = self.buffer.len() as u16;
            if self.offset >= buff_len {
                return Err(String::from("No bytes left to read")).expect("No bytes left to read");
            }
            Ok(self.buffer[(self.offset as usize)..].to_vec())
        }

        pub fn put(&mut self, value: Vec<u8>) {
            self.buffer.extend(value);
        }

        pub fn get_bool(&mut self) -> bool {
            let byte = self.get(1);
            match byte {
                Ok(b) => {
                    b[0] != 0
                }
                Err(err) => {
                    println!("Error get_bool(): {}", err);
                    false
                }
            }
        }

        pub fn put_bool(&mut self, value: bool) {
            let byte = if value { 0x01 } else { 0x00 };
            self.buffer.push(byte);
        }

        pub fn get_byte(&mut self) -> u8 {
            let byte = self.get(1);
            match byte {
                Ok(b) => {
                    b[0]
                }
                Err(err) => {
                    println!("Error get_byte(): {}", err);
                    0
                }
            }
        }

        pub fn put_byte(&mut self, value: u8) {
            self.buffer.push(value);
        }

        pub fn get_short(&mut self) -> u16 {
            let bytes = self.get(2);


            match bytes {
                Ok(byte) => {
                    let high_byte = byte[0] as u16;
                    let low_byte = byte[1] as u16;
                    (high_byte << 8) | low_byte
                }
                Err(err) => {
                    println!("Error get_short(): {}", err);
                    0
                }
            }
        }

        pub fn get_signed_short(&mut self) -> i16 {
            let bytes = self.get(2);
            match bytes {
                Ok(byte) => {
                    i16::from_le_bytes([byte[0], byte[1]])
                }
                Err(err) => {
                    println!("Error get_signed_short(): {}", err);
                    0
                }
            }
        }

        pub fn put_short(&mut self, value: u16) {
            let short_bytes: [u8; 2] = value.to_le_bytes();
            self.buffer.push(short_bytes[0]);
            self.buffer.push(short_bytes[1]);
        }

        pub fn get_l_short(&mut self) -> u16 {
            let bytes = self.get(2);

            match bytes {
                Ok(byte) => {
                    let low_byte = byte[0] as u16;
                    let high_byte = byte[1] as u16;
                    low_byte | (high_byte << 8)
                }
                Err(err) => {
                    println!("Error get_l_short(): {}", err);
                    0
                }
            }
        }

        pub fn get_signed_l_short(&mut self) -> i16 {
            let bytes = self.get(2);

            match bytes {
                Ok(byte) => {
                    let low_byte = byte[0] as i16;
                    let high_byte = byte[1] as i16;
                    (high_byte << 8) | low_byte
                }
                Err(err) => {
                    println!("Error get_signed_l_short(): {}", err);
                    0
                }
            }
        }

        pub fn put_l_short(&mut self, value: u16) {
            self.buffer.push((value & 0xFF) as u8);       // Low byte
            self.buffer.push((value >> 8) as u8);        // High byte
        }

        pub fn get_triad(&mut self) -> i32 {
            let bytes = self.get(3);

            match bytes {
                Ok(byte) => {
                    ((byte[0] as i32) << 16) | ((byte[1] as i32) << 8) | (byte[2] as i32)
                }
                Err(err) => {
                    println!("Error get_triad(): {}", err);
                    0
                }
            }
        }

        pub fn put_triad(&mut self, value: i32) {
            self.buffer.push(((value >> 16) & 0xFF) as u8);
            self.buffer.push(((value >> 8) & 0xFF) as u8);
            self.buffer.push((value & 0xFF) as u8);
        }

        pub fn get_l_triad(&mut self) -> i32 {
            let bytes = self.get(3);

            match bytes {
                Ok(byte) => {
                    (byte[0] as i32) | ((byte[1] as i32) << 8) | ((byte[2] as i32) << 16)
                }
                Err(err) => {
                    println!("Error get_l_triad(): {}", err);
                    0
                }
            }
        }

        pub fn put_l_triad(&mut self, value: i32) {
            self.buffer.push((value & 0xFF) as u8);
            self.buffer.push(((value >> 8) & 0xFF) as u8);
            self.buffer.push(((value >> 16) & 0xFF) as u8);
        }

        pub fn get_int(&mut self) -> u32 {
            let bytes = self.get(4);
            match bytes {
                Ok(byte) => {
                    u32::from_be_bytes([byte[0], byte[1], byte[2], byte[3]])
                }
                Err(err) => {
                    println!("Error get_int(): {}", err);
                    0
                }
            }
        }

        pub fn put_int(&mut self, value: u32) {
            let bytes: [u8; 4] = value.to_be_bytes();
            self.buffer.extend_from_slice(&bytes);
        }

        pub fn get_l_int(&mut self) -> u32 {
            let bytes = self.get(4);
            match bytes {
                Ok(byte) => {
                    u32::from_le_bytes([byte[0], byte[1], byte[2], byte[3]])
                }
                Err(err) => {
                    println!("Error get_l_int(): {}", err);
                    0
                }
            }
        }

        pub fn put_l_int(&mut self, value: u32) {
            let bytes: [u8; 4] = value.to_le_bytes();
            self.buffer.extend_from_slice(&bytes);
        }

        pub fn get_float(&mut self) -> f32 {
            let bytes = self.get(4);

            match bytes {
                Ok(byte) => {
                    f32::from_be_bytes([byte[0], byte[1], byte[2], byte[3]])
                }
                Err(err) => {
                    println!("Error get_float(): {}", err);
                    0.0
                }
            }
        }

        /*pub fn get_rounded_float(&mut self, accuracy: usize) -> Result<f32, String> {
            let bytes = self.get(4);

            match bytes {
                Ok(byte) => {
                    f32::from_be_bytes([byte[0], byte[1], byte[2], byte[3]])
                }
                Err(err) => {
                    println!("Error get_int(): {}", err);
                    0.0
                }
            }
            Self::read_rounded_float(&bytes, accuracy)
        }*/
        /* fn get_rounded_l_float(&mut self, accuracy: usize) -> f32 {
            let bytes = self.get(4)?; // Get 4 bytes from the stream
            let value = Self::read_lfloat(bytes)?;
            Ok((value * 10f32.powf(accuracy as f32)).round() / 10f32.powf(accuracy as f32))
        }*/

        pub fn put_float(&mut self, value: f32) {
            let bytes: [u8; 4] = value.to_be_bytes();
            self.buffer.extend_from_slice(&bytes);
        }

        pub fn get_l_float(&mut self) -> f32 {
            let bytes = self.get(4);

            match bytes {
                Ok(byte) => {
                    f32::from_le_bytes([byte[0], byte[1], byte[2], byte[3]])
                }
                Err(err) => {
                    println!("Error get_l_float(): {}", err);
                    0.0
                }
            }
        }

        pub fn put_l_float(&mut self, value: f32) {
            let bytes: [u8; 4] = value.to_le_bytes();
            self.buffer.extend_from_slice(&bytes);
        }

        pub fn get_double(&mut self) -> f64 {
            let bytes = self.get(8);

            match bytes {
                Ok(byte) => {
                    f64::from_be_bytes([byte[0], byte[1], byte[2], byte[3], byte[4], byte[5], byte[6], byte[7]])
                }
                Err(err) => {
                    println!("Error get_double(): {}", err);
                    0.0
                }
            }
        }

        pub fn put_double(&mut self, value: f64) {
            let bytes: [u8; 8] =  value.to_be_bytes();
            self.buffer.extend_from_slice(&bytes);
        }

        pub fn get_l_double(&mut self) -> f64 {
            let bytes = self.get(8);

            match bytes {
                Ok(byte) => {
                    f64::from_le_bytes([byte[0], byte[1], byte[2], byte[3], byte[4], byte[5], byte[6], byte[7]])
                }
                Err(err) => {
                    println!("Error get_l_double(): {}", err);
                    0.0
                }
            }
        }

        pub fn put_l_double(&mut self, value: f64) {
            let bytes: [u8; 8] = value.to_le_bytes();
            self.buffer.extend_from_slice(&bytes);
        }

        pub fn get_long(&mut self) -> i64 {
            let bytes = self.get(8);

            match bytes {
                Ok(byte) => {
                    i64::from_be_bytes([byte[0], byte[1], byte[2], byte[3], byte[4], byte[5], byte[6], byte[7]])
                }
                Err(err) => {
                    println!("Error get_long(): {}", err);
                    0
                }
            }
        }

        pub fn put_long(&mut self, value: i64) {
            let bytes: [u8; 8] = value.to_be_bytes();
            self.buffer.extend_from_slice(&bytes);
        }

        pub fn get_l_long(&mut self) -> i64 {
            let bytes = self.get(8);

            match bytes {
                Ok(byte) => {
                    i64::from_le_bytes([byte[0], byte[1], byte[2], byte[3], byte[4], byte[5], byte[6], byte[7]])
                }
                Err(err) => {
                    println!("Error get_l_long(): {}", err);
                    0
                }
            }
        }

        pub fn put_l_long(&mut self, value: i64) {
            let bytes: [u8; 8] = value.to_le_bytes();
            self.buffer.extend_from_slice(&bytes);
        }

        pub fn get_unsigned_var_int(&mut self) -> u32 {
            let bytes = self.get(5);

            match bytes {
                Ok(byte) => {
                    let mut value = 0u32;
                    for i in 0..5 {
                        let b = byte[i];
                        value |= ((b & 0x7f) as u32) << (i * 7);
                        if b & 0x80 == 0 {
                            return value;
                        }
                    }
                    0
                }
                Err(err) => {
                    println!("Error get_unsigned_var_int(): {}", err);
                    0
                }
            }
        }
        pub fn put_unsigned_var_int(&mut self, mut value: u32) {
            while value > 0x7f {
                self.buffer.push((value & 0x7f) as u8 | 0x80);
                value >>= 7;
            }

            self.buffer.push(value as u8);
        }

        pub fn get_var_int(&mut self) -> i32 {
            let raw: u32 = self.get_unsigned_var_int();
            (((raw << 31) >> 31) ^ raw >> 1 ^ (raw & (1 << 31))) as i32
        }

        pub fn put_var_int(&mut self, value: i32) {
            let value: i32 = (value << 1) ^ (value >> 31);
            self.put_unsigned_var_int(value as u32);
        }

        pub fn get_unsigned_var_long(&mut self) -> u64 {
            let bytes = self.get(10);

            match bytes {
                Ok(byte) => {
                    let mut value = 0u64;
                    for i in 0..10 {
                        let b = byte[i];
                        value |= ((b & 0x7f) as u64) << (i * 7);
                        if b & 0x80 == 0 {
                            return value;
                        }
                    }
                    0
                }
                Err(err) => {
                    println!("Error get_unsigned_var_long(): {}", err);
                    0
                }
            }
        }

        pub fn put_unsigned_var_long(&mut self, mut value: u64) {
            let mut buf = Vec::new();
            while value > 0x7f {
                buf.push((value & 0x7f) as u8 | 0x80);
                value >>= 7;
            }
            buf.push(value as u8);
        }

        pub fn get_var_long(&mut self) -> i64 {
            let raw = self.get_unsigned_var_long();
            (((raw << 63) >> 63) ^ raw >> 1 ^ (raw & (1 << 63))) as i64
        }

        pub fn put_var_long(&mut self, value: i64) {
            let value: i64 = (value << 1) ^ (value >> 63);
            self.put_unsigned_var_long(value as u64);
        }

        /// Returns whether the offset has reached the end of the buffer.
        pub fn feof(&self) -> bool {
            self.offset >= self.buffer.len() as u16
        }
    }
}
