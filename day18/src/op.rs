use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
pub enum Value {
    Reg(char),
    Const(isize)
}

impl FromStr for Value {
    type Err = String;

    fn from_str(input: &str) -> Result<Value, Self::Err> {
        match input.chars().next().unwrap() {
            ch @ 'a' ... 'z' => {
                Ok(Value::Reg(ch))
            }

            _ => {
                match input.parse() {
                    Ok(n) => Ok(Value::Const(n)),
                    _ => Err(format!("Unable to parse: {}", input))
                }
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Op {
    Add(Value, Value),
    Jgz(Value, Value),
    Mod(Value, Value),
    Mul(Value, isize),
    Rcv(Value),
    Set(Value, Value),
    Snd(Value)
}

impl FromStr for Op {
    type Err = String;

    fn from_str(input: &str) -> Result<Op, Self::Err> {
        let mut tokens = input.split_whitespace();

        macro_rules! parse_two {
            ($op: expr) => {
                {
                    let a = tokens.next().expect("Arg A");
                    let b = tokens.next().expect("Arg B");

                    let a = a.parse().expect("Arg A");
                    let b = b.parse().expect("Arg B");

                    Ok($op(a, b))
                }
            }
        }

        match tokens.next().expect("Operation Name") {
            "add" => { parse_two!(Op::Add) }
            "jgz" => { parse_two!(Op::Jgz) }
            "mod" => { parse_two!(Op::Mod) }
            "mul" => { parse_two!(Op::Mul) }
            "set" => { parse_two!(Op::Set) }

            "rcv" => {
                let value = tokens.next().expect("Arg");
                let value = value.parse().expect("Arg");
                Ok(Op::Rcv(value))
            }

            "snd" => {
                let value = tokens.next().expect("Arg");
                let value = value.parse().expect("Arg");
                Ok(Op::Snd(value))
            }

            _ => Err(format!("Unable to parse: {}", input))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_from_str() {
        assert_eq!("a".parse(), Ok(Value::Reg('a')));
        assert_eq!("p".parse(), Ok(Value::Reg('p')));
        assert_eq!("12345".parse(), Ok(Value::Const(12345)));
        assert_eq!("-12345".parse(), Ok(Value::Const(-12345)));
    }

    #[test]
    fn test_op_from_str() {
        assert_eq!("add a -1".parse(), Ok(Op::Add(Value::Reg('a'), Value::Const(-1))));
        assert_eq!("add p b".parse(), Ok(Op::Add(Value::Reg('p'), Value::Reg('b'))));

        assert_eq!("jgz 1 3".parse(), Ok(Op::Jgz(Value::Const(1), Value::Const(3))));
        assert_eq!("jgz a -19".parse(), Ok(Op::Jgz(Value::Reg('a'), Value::Const(-19))));
        assert_eq!("jgz p p".parse(), Ok(Op::Jgz(Value::Reg('p'), Value::Reg('p'))));

        assert_eq!("mod b 10000".parse(), Ok(Op::Mod(Value::Reg('b'), Value::Const(10000))));
        assert_eq!("mod p a".parse(), Ok(Op::Mod(Value::Reg('p'), Value::Reg('a'))));

        assert_eq!("mul a 2".parse(), Ok(Op::Mul(Value::Reg('a'), 2)));

        assert_eq!("rcv a".parse(), Ok(Op::Rcv(Value::Reg('a'))));

        assert_eq!("set a 1".parse(), Ok(Op::Set(Value::Reg('a'), Value::Const(1))));
        assert_eq!("set a b".parse(), Ok(Op::Set(Value::Reg('a'), Value::Reg('b'))));

        assert_eq!("snd a".parse(), Ok(Op::Snd(Value::Reg('a'))));
    }
}
