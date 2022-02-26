mod operations {
    pub fn adder(a: u32, b: u32) -> u32 {
        let (mut a, mut b) = (a, b);

        while b != 0 {
            (a, b) = _half_adder(a, b);
            b <<= 1;
        }

        a
    }

    pub fn multiplier(a: u32, b: u32) -> u32 {
        let mut result = 0;

        for _ in 0..b {
            result = adder(result, a);
        }

        result
    }

    pub fn gray_code(n: u32) -> u32 {
        let msb = _find_msb(n);
        let mut g_code = _set_ith_bit(0, msb, true);

        for i in 0..msb {
            let bin_bits: [bool; 2] = [_get_ith_bit(n, i), _get_ith_bit(n, i+1)];
            let g_bit = bin_bits[0] ^ bin_bits[1];
            g_code = _set_ith_bit(g_code, i, g_bit);
        }

        g_code
    }

    fn _half_adder(a: u32, b: u32) -> (u32, u32) {
        (a^b, a & b)
    }

    pub fn _find_msb(n: u32) -> u8 {
        for i in (0..=31).rev() {
            if _get_ith_bit(n, i) {
                return i
            }
        }

        0
    }

    fn _get_ith_bit(n: u32, i: u8) -> bool {
        n & ( 1 << i ) != 0
    }

    fn _set_ith_bit(n: u32, pos: u8, value: bool) -> u32 {
        n | ( ( value as u32 ) << pos)
    }
}

#[cfg(test)]
mod tests {
    use crate::operations::*;

    fn test_adder(x: u32, y: u32) {
        let expected = x + y;
        let result = adder(x, y);
        assert_eq!(expected, result);
    }

    fn test_multiplier(x: u32, y: u32) {
        let expected = x * y;
        let result = multiplier(x, y);
        assert_eq!(expected, result);
    }

    #[test]
    fn adder_tests() {
        test_adder(1,1);
        test_adder(3,6);
        test_adder(32405,21412421);
        test_adder(1321,2312);
        test_adder(868,3213123);
        test_adder(45679,4364367);
    }

    #[test]
    fn multiplier_tests() {
        test_multiplier(1, 1);
        test_multiplier(3, 6);
        test_multiplier(233, 6213);
        test_multiplier(6533, 624);
        test_multiplier(322309, 6543);
    }

    #[test]
    fn gray_code_tests() {
        let g_code = gray_code(4);
        assert_eq!(6, g_code);
        let g_code = gray_code(7);
        assert_eq!(4, g_code);
    }
}
