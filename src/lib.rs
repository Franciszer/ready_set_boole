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

    pub fn eval_formula(formula: &str) -> bool {
        let formula = _lex(formula);
        true
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
    enum Operation {
        One(SimpleOpType),
        Two(OpType)
    }

    struct LexedExpr {
        operands: Vec<bool>,
        operations: Vec<Operation>
    }

    fn _lex_operands(formula: &str) -> (Vec<bool>, usize) {
        let mut operation_idx= 0;
        let mut operands = Vec::new();
        for i in 0..formula.len() {
            match formula.chars().nth(i).unwrap() {
                '1' => {
                    operands.push(true);
                }
                '2' => {
                    operands.push(false);
                }
                _ => {
                    operation_idx = i;
                    break ;
                }
            }
        }
        (operands, operation_idx)
    }

    fn _lex_operations(formula: &str, operation_idx: usize) -> Result<Vec<Operation>, String> {
        let mut count = 0;
        let slice= &formula[operation_idx..formula.len()];
        let mut operations = Vec::with_capacity(formula.len() - operation_idx);
        for token in slice.chars() {
            match token {
                '!' => {
                    count += 1;
                    operations.push(Operation::One(_negation));
                },
                x => {
                    count += 2;
                    match x {
                        '&' => { operations.push(Operation::Two(_conjunction)); }
                        '|' => { operations.push(Operation::Two(_disjunction)); }
                        '^' => { operations.push(Operation::Two(_exclusive_disjunction)); }
                        '>' => { operations.push(Operation::Two(_material_condition)); }
                        '=' => { operations.push(Operation::Two(_logical_evidence)); }
                        x => { return Err(format!("Expected operation, got {}", x)); }
                    }
                }
            }
        }
        return Ok(operations)
    }

    fn _lex(formula: &str) -> Result<LexedExpr, String> {
        let (operands, operation_idx) = _lex_operands(formula);

        let operations = _lex_operations(formula, operation_idx);
        match operations {
            Ok(x) => Ok(LexedExpr{
                operands,
                operations: x
            }),
            Err(e) => Err(e)
        }
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
