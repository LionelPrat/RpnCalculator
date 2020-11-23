
pub fn rpn(a: &str) -> Result<f64, String> {
    let vec = parse_split(a);

    if vec.len() == 1 && vec[0] == "" {
        return Ok(0.);
    }

    let mut stack = Vec::<f64>::new();
    for op in vec.iter() {
        match op {
            &"*" =>{
                if stack.len() < 2 {
                    return Err(format!("format error before *, stack contains less than 2 elements"))
                }
                let res = stack.pop().unwrap() * stack.pop().unwrap();
                stack.push(res);
            },
            &"/" => {
                if stack.len() < 2 {
                    return Err(format!("format error before /, stack contains less than 2 elements"))
                }
                let last = stack.pop().unwrap();
                let previous = stack.pop().unwrap();
                let res = previous / last;
                stack.push(res);
            },
            &"+" => {
                if stack.len() < 2 {
                    return Err(format!("format error before +, stack contains less than 2 elements"))
                }
                let res = stack.pop().unwrap() + stack.pop().unwrap();
                stack.push(res);
            },
            &"-" => {
                if stack.len() < 2 {
                    return Err(format!("format error before -, stack contains less than 2 elements"))
                }
                let last = stack.pop().unwrap();
                let previous = stack.pop().unwrap();
                let res = previous - last;
                stack.push(res);
            },
            &"SQRT" => {
                if stack.is_empty() {
                    return Err(format!("format error before SQRT, stack was empty"))
                }

                let last = stack.pop().unwrap();
                stack.push(last.sqrt());
            },
            &"MAX" => {
                if stack.len() < 2 {
                    return Err(format!("format error before MAX, stack contains less than 2 elements"))
                }

                let last = stack.pop().unwrap();
                let previous = stack.pop().unwrap();
                let res = if previous > last { previous } else { last };
                stack.push(res);
            },
            _ => {
                match op.parse::<f64>() {
                    Ok(i) => stack.push(i),
                    Err(..) => return Err(format!("this was not an integer {}", op))
                }
            }
        }
    }

    if stack.len() != 1 {
         return Err(format!("format error, stack should contain one element only"))
    }

    return Ok(stack.pop().unwrap());
}

pub fn parse_split(a: &str) -> Vec<&str> {
    let vec: Vec<&str> = a.trim().split(" ").collect();
    return vec;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    
    #[test]
    fn rpn_empty() {
        let sample = "";
        should_be_ok(0., rpn(sample));
    }

    #[test]
    fn rpn_int() {
        let sample = "42";
        should_be_ok(42., rpn(sample));
    }

    #[test]
    fn rpn_add() {
        let sample = "30 5 +";
        should_be_ok(35., rpn(sample));
    }

    #[test]
    fn rpn_minus() {
        let sample = "30 5 -";
        should_be_ok(25., rpn(sample));
    }

    #[test]
    fn rpn_multiply() {
        let sample = "30 5 *";
        should_be_ok(150., rpn(sample));
    }

    #[test]
    fn rpn_divide() {
        let sample = "30 5 /";
        should_be_ok(6., rpn(sample));
    }

    #[test]
    fn rpn_several() {
        let sample = "1 2 + 3 *";
        should_be_ok(9., rpn(sample));
    }

    #[test]
    fn rpn_several2() {
        let sample = "10 20 5 / +";
        should_be_ok(14., rpn(sample));
    }

    #[test]
    fn rpn_square_root() {
        let sample = "9 SQRT";
        should_be_ok(3., rpn(sample));
    }

    #[test]
    fn rpn_max() {
        let sample = "5 3 4 2 9 1 MAX";
        should_fail(rpn(sample));
    }

    #[test]
    fn rpn_max2() {
        let sample = "4 5 MAX 4 5 MAX 1 2 MAX * +";
        should_be_ok(15., rpn(sample));
    }
    
    #[test]
    fn rpn_wrong_param() {
        let sample = "toto";
        should_fail(rpn(sample));
    }

    #[test]
    fn rpn_wrong_number_of_params() {
        let sample = "5 +";
        should_fail(rpn(sample));
    }

    #[test]
    fn rpn_wrong_number_of_params_max() {
        let sample = "MAX";
        should_fail(rpn(sample));
    }

    #[test]
    fn rpn_wrong_number_of_params_sqrt() {
        let sample = "SQRT";
        should_fail(rpn(sample));
    }

    #[test]
    fn rpn_wrong_number_of_params_sqrt2() {
        let sample = "1 2 SQRT";
        should_fail(rpn(sample));
    }

    #[test]
    fn test_parse_multiple_operator() {
        let sample = "20 5 /";
        let expected = vec!("20", "5", "/");

        test_vector(expected, parse_split(sample));
    }

    fn should_be_ok(expected:f64, a: Result<f64, String>)
    {
        match a {
            Ok(i) => assert_eq!(expected, i),
            Err(..) => assert!(false, "this should not fail")
        }
    }

    fn should_fail(a: Result<f64, String>)
    {
        match a {
            Ok(..) => assert!(false, "this should fail"),
            Err(..) => assert!(true)
        }
    }

    fn test_vector(a: Vec<&str>, b: Vec<&str>)
    {
        assert_eq!(a.len(), b.len());

        for i in 0..=a.len()-1 {
            assert_eq!(a[i], b[i]);
        }
    }
}