#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn test_default_hexadecimal() {
        let bytes: Vec<u8> = "orange juice and banana peel! apple juice and lemon rind!".as_bytes().to_vec();
        // Multiple Lines, Odd Last Line
        let mut mult_iterator = DefaultHexadecimal::new(bytes.len(), &bytes);
        assert_eq!(mult_iterator.next().unwrap(), "0000000 726f 6e61 6567 6a20 6975 6563 6120 646e ".to_string());
        assert_eq!(mult_iterator.next().unwrap(), "0000010 6220 6e61 6e61 2061 6570 6c65 2021 7061 ".to_string());
        assert_eq!(mult_iterator.next().unwrap(), "0000020 6c70 2065 756a 6369 2065 6e61 2064 656c ".to_string());
        assert_eq!(mult_iterator.next().unwrap(), "0000030 6f6d 206e 6972 646e 0021 ".to_string());
        assert_eq!(mult_iterator.next(), None);
        // Full Last Line
        let mut full_iterator = DefaultHexadecimal::new(16, &bytes);
        assert_eq!(full_iterator.next().unwrap(), "0000000 726f 6e61 6567 6a20 6975 6563 6120 646e ".to_string());
        assert_eq!(full_iterator.next(), None);
        // Even Last Line
        let mut even_iterator = DefaultHexadecimal::new(30, &bytes);
        assert_eq!(even_iterator.next().unwrap(), "0000000 726f 6e61 6567 6a20 6975 6563 6120 646e ".to_string());
        assert_eq!(even_iterator.next().unwrap(), "0000010 6220 6e61 6e61 2061 6570 6c65 2021 ".to_string());
        assert_eq!(even_iterator.next(), None);
    }

    #[test]
    fn test_two_byte_hexadecimal() {
        let bytes: Vec<u8> = "orange juice and banana peel! apple juice and lemon rind!".as_bytes().to_vec();
        // Multiple Lines, Odd Last Line
        let mut mult_iterator = TwoByteHexadecimal::new(bytes.len(), &bytes);
        assert_eq!(mult_iterator.next().unwrap(), "0000000    726f    6e61    6567    6a20    6975    6563    6120    646e    ".to_string());
        assert_eq!(mult_iterator.next().unwrap(), "0000010    6220    6e61    6e61    2061    6570    6c65    2021    7061    ".to_string());
        assert_eq!(mult_iterator.next().unwrap(), "0000020    6c70    2065    756a    6369    2065    6e61    2064    656c    ".to_string());
        assert_eq!(mult_iterator.next().unwrap(), "0000030    6f6d    206e    6972    646e    0021    ".to_string());
        assert_eq!(mult_iterator.next(), None);
        // Full Last Line
        let mut full_iterator = TwoByteHexadecimal::new(16, &bytes);
        assert_eq!(full_iterator.next().unwrap(), "0000000    726f    6e61    6567    6a20    6975    6563    6120    646e    ".to_string());
        assert_eq!(full_iterator.next(), None);
        // Even Last Line
        let mut even_iterator = TwoByteHexadecimal::new(30, &bytes);
        assert_eq!(even_iterator.next().unwrap(), "0000000    726f    6e61    6567    6a20    6975    6563    6120    646e    ".to_string());
        assert_eq!(even_iterator.next().unwrap(), "0000010    6220    6e61    6e61    2061    6570    6c65    2021    ".to_string());
        assert_eq!(even_iterator.next(), None);
    }

    #[test]
    fn test_two_byte_decimal() {
        let bytes: Vec<u8> = "orange juice and banana peel! apple juice and lemon rind!".as_bytes().to_vec();
        // Multiple Lines, Odd Last Line
        let mut mult_iterator = TwoByteDecimal::new(bytes.len(), &bytes);
        assert_eq!(mult_iterator.next().unwrap(), "0000000   29295   28257   25959   27168   26997   25955   24864   25710   ".to_string());
        assert_eq!(mult_iterator.next().unwrap(), "0000010   25120   28257   28257   08289   25968   27749   08225   28769   ".to_string());
        assert_eq!(mult_iterator.next().unwrap(), "0000020   27760   08293   30058   25449   08293   28257   08292   25964   ".to_string());
        assert_eq!(mult_iterator.next().unwrap(), "0000030   28525   08302   26994   25710   00033   ".to_string());
        assert_eq!(mult_iterator.next(), None);
        // Full Last Line
        let mut full_iterator = TwoByteDecimal::new(16, &bytes);
        assert_eq!(full_iterator.next().unwrap(), "0000000   29295   28257   25959   27168   26997   25955   24864   25710   ".to_string());
        assert_eq!(full_iterator.next(), None);
        // Even Last Line
        let mut even_iterator = TwoByteDecimal::new(30, &bytes);
        assert_eq!(even_iterator.next().unwrap(), "0000000   29295   28257   25959   27168   26997   25955   24864   25710   ".to_string());
        assert_eq!(even_iterator.next().unwrap(), "0000010   25120   28257   28257   08289   25968   27749   08225   ".to_string());
        assert_eq!(even_iterator.next(), None);
    }

    #[test]
    fn test_two_byte_octal() {
        let bytes: Vec<u8> = "orange juice and banana peel! apple juice and lemon rind!".as_bytes().to_vec();
        // Multiple Lines, Odd Last Line
        let mut mult_iterator = TwoByteOctal::new(bytes.len(), &bytes);
        assert_eq!(mult_iterator.next().unwrap(), "0000000  071157  067141  062547  065040  064565  062543  060440  062156  ".to_string());
        assert_eq!(mult_iterator.next().unwrap(), "0000010  061040  067141  067141  020141  062560  066145  020041  070141  ".to_string());
        assert_eq!(mult_iterator.next().unwrap(), "0000020  066160  020145  072552  061551  020145  067141  020144  062554  ".to_string());
        assert_eq!(mult_iterator.next().unwrap(), "0000030  067555  020156  064562  062156  000041  ".to_string());
        assert_eq!(mult_iterator.next(), None);
        // Full Last Line
        let mut full_iterator = TwoByteOctal::new(16, &bytes);
        assert_eq!(full_iterator.next().unwrap(), "0000000  071157  067141  062547  065040  064565  062543  060440  062156  ".to_string());
        assert_eq!(full_iterator.next(), None);
        // Even Last Line
        let mut even_iterator = TwoByteOctal::new(30, &bytes);
        assert_eq!(even_iterator.next().unwrap(), "0000000  071157  067141  062547  065040  064565  062543  060440  062156  ".to_string());
        assert_eq!(even_iterator.next().unwrap(), "0000010  061040  067141  067141  020141  062560  066145  020041  ".to_string());
        assert_eq!(even_iterator.next(), None);
    }

    #[test]
    fn test_one_byte_octal() {
        let bytes: Vec<u8> = "orange juice and banana peel! apple juice and lemon rind!".as_bytes().to_vec();
        // Multiple Lines, Odd Last Line
        let mut mult_iterator = OneByteOctal::new(bytes.len(), &bytes);
        assert_eq!(mult_iterator.next().unwrap(), "0000000 157 162 141 156 147 145 040 152 165 151 143 145 040 141 156 144 ".to_string());
        assert_eq!(mult_iterator.next().unwrap(), "0000010 040 142 141 156 141 156 141 040 160 145 145 154 041 040 141 160 ".to_string());
        assert_eq!(mult_iterator.next().unwrap(), "0000020 160 154 145 040 152 165 151 143 145 040 141 156 144 040 154 145 ".to_string());
        assert_eq!(mult_iterator.next().unwrap(), "0000030 155 157 156 040 162 151 156 144 041 ".to_string());
        assert_eq!(mult_iterator.next(), None);
        // Full Last Line
        let mut full_iterator = OneByteOctal::new(16, &bytes);
        assert_eq!(full_iterator.next().unwrap(), "0000000 157 162 141 156 147 145 040 152 165 151 143 145 040 141 156 144 ".to_string());
        assert_eq!(full_iterator.next(), None);
        // Even Last Line
        let mut even_iterator = OneByteOctal::new(30, &bytes);
        assert_eq!(even_iterator.next().unwrap(), "0000000 157 162 141 156 147 145 040 152 165 151 143 145 040 141 156 144 ".to_string());
        assert_eq!(even_iterator.next().unwrap(), "0000010 040 142 141 156 141 156 141 040 160 145 145 154 041 040 ".to_string());
        assert_eq!(even_iterator.next(), None);
    }

    #[test]
    fn test_one_byte_char() {
        let bytes: Vec<u8> = "orange juice and banana peel! apple juice and lemon rind!".as_bytes().to_vec();
        // Multiple Lines, Odd Last Line
        let mut mult_iterator = OneByteChar::new(bytes.len(), &bytes);
        assert_eq!(mult_iterator.next().unwrap(), "0000000   o   r   a   n   g   e       j   u   i   c   e       a   n   d   ".to_string());
        assert_eq!(mult_iterator.next().unwrap(), "0000010       b   a   n   a   n   a       p   e   e   l   !       a   p   ".to_string());
        assert_eq!(mult_iterator.next().unwrap(), "0000020   p   l   e       j   u   i   c   e       a   n   d       l   e   ".to_string());
        assert_eq!(mult_iterator.next().unwrap(), "0000030   m   o   n       r   i   n   d   !   ".to_string());
        assert_eq!(mult_iterator.next(), None);
        // Full Last Line
        let mut full_iterator = OneByteChar::new(16, &bytes);
        assert_eq!(full_iterator.next().unwrap(), "0000000   o   r   a   n   g   e       j   u   i   c   e       a   n   d   ".to_string());
        assert_eq!(full_iterator.next(), None);
        // Even Last Line
        let mut even_iterator = OneByteChar::new(30, &bytes);
        assert_eq!(even_iterator.next().unwrap(), "0000000   o   r   a   n   g   e       j   u   i   c   e       a   n   d   ".to_string());
        assert_eq!(even_iterator.next().unwrap(), "0000010       b   a   n   a   n   a       p   e   e   l   !       ".to_string());
        assert_eq!(even_iterator.next(), None);
    }

}