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
}
