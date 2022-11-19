
pub fn encode_str(input: &str) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::new();
    let mut prevchar: char  = input.chars().nth(0).unwrap();
    let mut charcount: u8   = 0;

    for c in input.chars() {
        if c == prevchar { 
            charcount = charcount + 1;
            continue;
        }

        output.push(prevchar as u8);
        output.push(charcount);
        prevchar = c;
        charcount = 1;
    }

    if charcount > 0 { // push last character count into output
        output.push(prevchar as u8);
        output.push(charcount);
    }

    return output;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_str() {
        assert_eq!(encode_str("aaaaabbbbcccdda"), [97, 5, 98, 4, 99, 3, 100, 2, 97, 1]);
    }
}