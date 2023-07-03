fn main() {
    let hex = String::from("AFDA");
    let b64 = hto64(hex);
    println!("{}", b64);
}

fn hto64(hex: String) -> String {
    let mut b64 = String::new();
    let charset: [char; 64] = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'];

    for (i, char) in hex.chars().rev().enumerate() {
        let mut index: i32 = char.to_digit(16).unwrap().try_into().unwrap();
        index = index * (16_i32.pow(i as u32));
        //println!("{} {}", i, (index as u8));
        b64.push(charset[(index%64) as usize]);
    }
    b64
}