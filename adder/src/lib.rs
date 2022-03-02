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
            let bin_bits: [bool; 2] = [_get_ith_bit(n, i), _get_ith_bit(n, i + 1)];
            let g_bit = bin_bits[0] ^ bin_bits[1];
            g_code = _set_ith_bit(g_code, i, g_bit);
        }

        g_code
    }

    type SimpleOpType = fn(bool) -> bool;

    type OpType = fn(bool, bool) -> bool;

    fn _negation(a: bool) -> bool { !a }

    fn _conjunction(a: bool, b: bool) -> bool { a & b }

    fn _disjunction(a: bool, b: bool) -> bool { a | b }

    fn _exclusive_disjunction(a: bool, b: bool) -> bool { a ^ b }

    fn _material_condition(a: bool, b: bool) -> bool { !a | b }

    fn _logical_evidence(a: bool, b: bool) -> bool { a == b }

    // Operand is either true or false
    // Simple operations take 1 argument
    // Regular operations take 2

    enum Token {
        Operand(bool),
        Operation(SimpleOpType),
        Operation2(OpType)
    }

    fn _lex_formula(formula: &str) -> Result<Vec<Token>, String> {
        formula.chars().map(
          |x| match x {
              '1' => Ok(Token::Operand(true)),
              '0' => Ok(Token::Operand(false)),
              '!' => Ok(Token::Operation(_negation)),
              '&' => Ok(Token::Operation2(_conjunction)),
              '|' => Ok(Token::Operation2(_disjunction)),
              '^' => Ok(Token::Operation2(_exclusive_disjunction)),
              '>' => Ok(Token::Operation2(_material_condition)),
              '=' => Ok(Token::Operation2(_logical_evidence)),
              unknown => { return Err(format!("Unexpected Symbol {}", unknown)); }
          }
        ).collect()
    }

    // fn _find_operation(tokens: &Vec<Token>) -> Option<&Token> {
    //     tokens.iter().find(|x| match x {
    //         Token::Operation(x) => true,
    //         Token::SimpleOperation(op) => true,
    //         _ => false
    //     })
    // }

    fn _parse_operands(formula: &Vec<Token>) -> (Vec<bool>, usize) {
        let mut operands= Vec::new();

        let mut idx = 0;
        for i in 0..formula.len() {
            match &formula[i] {
                Token::Operand(x) => operands.push(x.clone()),
                Token => {
                    idx = i;
                    break
                }
            };
        }
        (operands, idx)
    }

    fn _parse_operations(formula: Vec<Token>, idx: usize) -> Result<Vec<Token>, String> {
        let mut operations = Vec::new();


        for i in formula.into_iter() {
            match token {
                Token::Operation(x) => operations.push(Token::Operation(x)),
                Token::Operation2(x) => operations.push(Token::Operation2(x)),
                Token::Operand(x) => {
                    return Err(format!("Expected operation, found operand {}", x));
                }
            }
        }

        operations
    }

    fn _parse_formula(formula: &Vec<Token>) -> Result<bool, String> {
        let (operands, idx) = _parse_operands(formula);

        if idx == 0 {
            return Err(String::from("Expected operand"));
        }

        // let operators: Vec<Token>;
        // for i in idx..formula.len() {
        //     let operand = match formula[i] {
        //         Token::Operation2(x) => operators.push(Token::Operation2(x)),
        //         Token::Operation(x) => operators.push(Token::Operation(x),
        //     };
        // }
        Ok(true)
    }

    pub fn eval_formula(formula: &str) {
        // let tokens = _lex_formula(formula);
        // let a = tokens;
    }

    fn _half_adder(a: u32, b: u32) -> (u32, u32) {
        (a ^ b, a & b)
    }

    pub fn _find_msb(n: u32) -> u8 {
        for i in (0..=31).rev() {
            if _get_ith_bit(n, i) {
                return i;
            }
        }

        0
    }

    fn _get_ith_bit(n: u32, i: u8) -> bool {
        n & (1 << i) != 0
    }

    fn _set_ith_bit(n: u32, pos: u8, value: bool) -> u32 {
        n | ((value as u32) << pos)
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
        test_adder(1, 1);
        test_adder(3, 6);
        test_adder(32405, 21412421);
        test_adder(1321, 2312);
        test_adder(868, 3213123);
        test_adder(45679, 4364367);
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

    #[test]
    fn eval_formula_tests() {
        eval_formula("1010|&=");
    }
}
