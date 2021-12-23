pub fn bitvec_to_decimal(v: &[u8]) -> u8 {
    let mut decimal = 0;

    let base: u8 = 2;
    let mut _index = 8;
    let mut exp = 0;
    while _index > 0 {
        let result_exp = base.pow(exp);
        _index -= 1;
        decimal += v[_index] * result_exp;
        exp += 1;
    }
    decimal
}

pub fn decimal_to_bitvec(decimal: u8) -> [u8; 8] {
    let mut decimal = decimal;
    let mut bit_array = [0u8; 8];

    let mut idx = bit_array.len()-1;
    while decimal > 1 {
        let rest = decimal % 2;
        if rest == 1 {
            decimal = (decimal-1)/2;
        } else {
            decimal /= 2;
        }
        bit_array[idx] = rest;
        idx -= 1;
    }
    bit_array[idx] = decimal;
    bit_array
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitvec_to_decinal() {
        assert_eq!(2,   bitvec_to_decimal(&vec![0, 0, 0, 0, 0, 0, 1, 0]));
        assert_eq!(8,   bitvec_to_decimal(&vec![0, 0, 0, 0, 1, 0, 0, 0]));
        assert_eq!(42,  bitvec_to_decimal(&vec![0, 0, 1, 0, 1, 0, 1, 0]));
        assert_eq!(250, bitvec_to_decimal(&vec![1, 1, 1, 1, 1, 0, 1, 0]));
    }

    #[test]
    fn test_decimal_to_binary() {
        assert_eq!([0, 0, 0, 0, 0, 0, 1, 0], decimal_to_bitvec(2));
        assert_eq!([0, 0, 0, 0, 1, 0, 0, 0], decimal_to_bitvec(8));
        assert_eq!([0, 0, 0, 1, 0, 0, 0, 1], decimal_to_bitvec(17));
        assert_eq!([0, 0, 1, 0, 1, 0, 1, 0], decimal_to_bitvec(42));
        assert_eq!([1, 1, 0, 0, 0, 0, 0, 0], decimal_to_bitvec(192));
    }
}
