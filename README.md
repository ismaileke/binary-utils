# binary-utils
A binary system to be used for RakNet protocol.

## Usage

📄Cargo.toml
```css
[dependencies]
binary-utils = { git = "https://github.com/ismaileke/binary-utils.git", branch = "master"}
```


📄main.rs
```rust
use binary_utils::binary::Stream;

fn main() {
    let mut stream = Stream::new(vec![1, 2], 0);
    stream.put_byte(128);
    stream.put_l_triad(12345);

    let _ = stream.get_byte(); // first byte -> 1
    let _ = stream.get_byte(); // second byte -> 2
    let _ = stream.get_byte(); // third byte -> 128
    let triad_num = stream.get_l_triad(); // triad number -> 12345

    println!("{}", triad_num); //  12345
    println!("{:?}", stream.get_buffer()); // [1, 2, 128, 57, 48, 0]
}
```

## 📍 NOTE
It is still in development.
