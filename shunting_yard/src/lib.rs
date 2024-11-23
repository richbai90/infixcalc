use core::fmt;
use std::f64::NAN;

#[derive(Copy, Clone, Debug)]
pub enum Operator {
    Add,
    CloseParen,
    Div,
    OpenParen,
    Pow,
    Mult,
    Sub,
    Mod,
    Exp,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OperatorConversionError;
#[derive(Debug, Clone, PartialEq)]
pub struct PostfixEvalError;

impl fmt::Display for OperatorConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid character for operator")
    }
}

impl fmt::Display for PostfixEvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid postfix expression")
    }
}

impl TryFrom<char> for Operator {
    type Error = OperatorConversionError;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        let res = match value {
            '+' => Ok(Operator::Add),
            ')' => Ok(Operator::CloseParen),
            '/' => Ok(Operator::Div),
            'E' => Ok(Operator::Exp),
            '%' => Ok(Operator::Mod),
            '*' => Ok(Operator::Mult),
            '(' => Ok(Operator::OpenParen),
            '^' => Ok(Operator::Pow),
            '-' => Ok(Operator::Sub),
            _ => Err(OperatorConversionError),
        };

        return res;
    }
}

impl From<Operator> for char {
    fn from(value: Operator) -> Self {
        match value {
            Operator::Add => '+',
            Operator::CloseParen => ')',
            Operator::Div => '/',
            Operator::Exp => 'E',
            Operator::Mod => '%',
            Operator::Mult => '*',
            Operator::OpenParen => '(',
            Operator::Pow => '^',
            Operator::Sub => '-',
        }
    }
}

impl std::cmp::PartialEq for Operator {
    fn eq(&self, other: &Self) -> bool {
        return self.precidence().eq(&other.precidence());
    }
}

impl std::cmp::Eq for Operator {}
impl std::cmp::PartialOrd for Operator {
    fn ge(&self, other: &Self) -> bool {
        return self.precidence().ge(&other.precidence());
    }
    fn le(&self, other: &Self) -> bool {
        return self.precidence().le(&other.precidence());
    }
    fn gt(&self, other: &Self) -> bool {
        return self.precidence().gt(&other.precidence());
    }
    fn lt(&self, other: &Self) -> bool {
        return self.precidence().lt(&other.precidence());
    }
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return self.precidence().partial_cmp(&other.precidence());
    }
}
impl std::cmp::Ord for Operator {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return u8::cmp(&self.precidence(), &other.precidence());
    }
}

impl Operator {
    fn precidence(&self) -> u8 {
        match self {
            Operator::Add => 1,
            Operator::Div => 2,
            Operator::Mod => 2,
            Operator::Mult => 2,
            Operator::Exp => 2,
            Operator::Pow => 3,
            Operator::Sub => 1,
            _ => 0,
        }
    }

    fn operate(&self, a: f64, b: f64) -> f64 {
        match self {
            Operator::Add => a + b,
            Operator::Div => a / b,
            Operator::Mod => a % b,
            Operator::Mult => a * b,
            Operator::Exp => a * 10.0_f64.powf(b),
            Operator::Pow => a.powf(b),
            Operator::Sub => a - b,
            _ => NAN,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Operator(Operator),
    Operand(String),
}

pub fn tokenize(expr: &str) -> Vec<Token> {
    let mut operators: Vec<Operator> = Vec::new();
    let mut pe: Vec<Token> = Vec::new();
    // prepare expression
    let prepped_expr = format!("({})", expr.replace(" ", ""));

    let mut operand: Vec<char> = vec![];
    for c in prepped_expr.chars() {
        match <Operator as TryFrom<char>>::try_from(c) {
            Ok(op) => {
                if !operand.is_empty() {
                    pe.push(Token::Operand(String::from_iter(operand.to_owned())));
                    operand.clear();
                }
                match op {
                    Operator::OpenParen => operators.push(op),
                    Operator::CloseParen => {
                        while let Some(nextop) = operators.pop() {
                            match nextop {
                                Operator::OpenParen => break,
                                _ => pe.push(Token::Operator(nextop)),
                            }
                        }
                    }
                    _ => {
                        while let Some(nextop) = operators.last() {
                            if *nextop == Operator::OpenParen || op > *nextop {
                                break;
                            }
                            pe.push(Token::Operator(operators.pop().unwrap()));
                        }

                        operators.push(op)
                    }
                }
            }
            Err(_) => {
                operand.push(c) // not an operator so must be operand
            }
        }
    }

    return pe;
}

pub fn evaluate(tokens: Vec<Token>) -> Result<f64, PostfixEvalError> {
    let mut operands: Vec<f64> = vec![];

    for token in tokens {
        match token {
            Token::Operand(op) => match op.parse::<f64>() {
                Ok(f) => operands.push(f),
                Err(_) => return Err(PostfixEvalError),
            },

            Token::Operator(op) => {
                if operands.len() < 2 {
                    return Err(PostfixEvalError);
                }
                let b = operands.pop().unwrap();
                let a = operands.pop().unwrap();

                let val = op.operate(a, b);
                if val.is_nan() {
                    return Err(PostfixEvalError);
                }
                operands.push(val);
            }
        }
    }

    if operands.len() != 1 {
        return Err(PostfixEvalError);
    }

    Ok(operands[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operator_try_from() {
        assert_eq!(Operator::try_from('+'), Ok(Operator::Add));
        assert_eq!(Operator::try_from(')'), Ok(Operator::CloseParen));
        assert_eq!(Operator::try_from('/'), Ok(Operator::Div));
        assert_eq!(Operator::try_from('*'), Ok(Operator::Mult));
        assert_eq!(Operator::try_from('('), Ok(Operator::OpenParen));
        assert_eq!(Operator::try_from('^'), Ok(Operator::Pow));
        assert_eq!(Operator::try_from('-'), Ok(Operator::Sub));
        assert!(Operator::try_from('a').is_err());
    }

    #[test]
    fn test_operator_into_char() {
        assert_eq!(char::from(Operator::Add), '+');
        assert_eq!(char::from(Operator::CloseParen), ')');
        assert_eq!(char::from(Operator::Div), '/');
        assert_eq!(char::from(Operator::Mult), '*');
        assert_eq!(char::from(Operator::OpenParen), '(');
        assert_eq!(char::from(Operator::Pow), '^');
        assert_eq!(char::from(Operator::Sub), '-');
    }

    #[test]
    fn test_operator_precedence() {
        assert!(Operator::Pow > Operator::Mult);
        assert!(Operator::Mult > Operator::Add);
        assert!(Operator::Div > Operator::Sub);
        assert!(Operator::Add == Operator::Sub);
        assert!(Operator::Mult == Operator::Div);
    }

    #[test]
    fn test_tokenize() {
        use self::Operator::*;
        use Token::*;

        assert_eq!(
            tokenize("a+b"),
            vec![
                Operand("a".to_string()),
                Operand("b".to_string()),
                Operator(Add),
            ]
        );

        assert_eq!(
            tokenize("a+b*c"),
            vec![
                Operand("a".to_string()),
                Operand("b".to_string()),
                Operand("c".to_string()),
                Operator(Mult),
                Operator(Add),
            ]
        );

        assert_eq!(
            tokenize("(a+b)*c"),
            vec![
                Operand("a".to_string()),
                Operand("b".to_string()),
                Operator(Add),
                Operand("c".to_string()),
                Operator(Mult),
            ]
        );

        assert_eq!(
            tokenize("a+b*c-d/e^f"),
            vec![
                Operand("a".to_string()),
                Operand("b".to_string()),
                Operand("c".to_string()),
                Operator(Mult),
                Operator(Add),
                Operand("d".to_string()),
                Operand("e".to_string()),
                Operand("f".to_string()),
                Operator(Pow),
                Operator(Div),
                Operator(Sub),
            ]
        );

        assert_eq!(
            tokenize("a.3+b.2*c.1-d/e^f"),
            vec![
                Operand("a.3".to_string()),
                Operand("b.2".to_string()),
                Operand("c.1".to_string()),
                Operator(Mult),
                Operator(Add),
                Operand("d".to_string()),
                Operand("e".to_string()),
                Operand("f".to_string()),
                Operator(Pow),
                Operator(Div),
                Operator(Sub),
            ]
        );

        assert_eq!(
            tokenize("a^(1/2)"),
            vec![
                Operand("a".to_string()),
                Operand("1".to_string()),
                Operand("2".to_string()),
                Operator(Div),
                Operator(Pow),
            ]
        );

        assert_eq!(
            tokenize("aE10*2"),
            vec![
                Operand("a".to_string()),
                Operand("10".to_string()),
                Operator(Exp),
                Operand("2".to_string()),
                Operator(Mult),
            ]
        );

        assert_eq!(
            tokenize("a%2 + 3"),
            vec![
                Operand("a".to_string()),
                Operand("2".to_string()),
                Operator(Mod),
                Operand("3".to_string()),
                Operator(Add),
            ]
        );
    }

    #[test]
    fn test_evaluate_simple() {
        use self::Operator::*;
        use Token::*;

        // Test basic arithmetic
        assert_eq!(
            evaluate(vec![
                Operand("3".to_string()),
                Operand("4".to_string()),
                Operator(Add)
            ]),
            Ok(7.0)
        );

        assert_eq!(
            evaluate(vec![
                Operand("10".to_string()),
                Operand("5".to_string()),
                Operator(Sub)
            ]),
            Ok(5.0)
        );

        assert_eq!(
            evaluate(vec![
                Operand("6".to_string()),
                Operand("3".to_string()),
                Operator(Mult)
            ]),
            Ok(18.0)
        );

        assert_eq!(
            evaluate(vec![
                Operand("15".to_string()),
                Operand("3".to_string()),
                Operator(Div)
            ]),
            Ok(5.0)
        );
    }

    #[test]
    fn test_evaluate_complex() {
        use self::Operator::*;
        use Token::*;
        // Test more complex expressions
        assert_eq!(
            evaluate(vec![
                Operand("3".to_string()),
                Operand("4".to_string()),
                Operand("2".to_string()),
                Operator(Mult),
                Operator(Add)
            ]),
            Ok(11.0)
        );

        assert_eq!(
            evaluate(vec![
                Operand("10".to_string()),
                Operand("2".to_string()),
                Operator(Pow)
            ]),
            Ok(100.0)
        );

        // Test decimal numbers
        assert_eq!(
            evaluate(vec![
                Operand("3.5".to_string()),
                Operand("2.5".to_string()),
                Operator(Add)
            ]),
            Ok(6.0)
        );

        // Test modulo
        assert_eq!(
            evaluate(vec![
                Operand("10".to_string()),
                Operand("3".to_string()),
                Operator(Mod)
            ]),
            Ok(1.0)
        );

        // Test exponentiation
        assert_eq!(
            evaluate(vec![
                Operand("2".to_string()),
                Operand("3".to_string()),
                Operator(Exp)
            ]),
            Ok(2000.0)
        );
    }

    #[test]
    fn test_evaluate_errors() {
        use self::Operator::*;
        use Token::*;

        // Test error case: invalid number
        assert!(evaluate(vec![Operand("not_a_number".to_string())]).is_err());

        // Test error case: insufficient operands
        assert!(evaluate(vec![Operand("5".to_string()), Operator(Add)]).is_err());

        // Test error case: too many operands
        assert!(evaluate(vec![
            Operand("5".to_string()),
            Operand("3".to_string()),
            Operand("2".to_string()),
            Operator(Add)
        ])
        .is_err());
    }
}
