
pub fn rpn(a: &str) -> f64 {
    let vec = parse_split(a);

    if vec.len() == 1 && vec[0] == "" {
        return 0.;
    }

    let mut stack = Vec::<f64>::new();
    for op in vec.iter() {
        match op {
            &"*" =>{
                let res = stack.pop().unwrap() * stack.pop().unwrap();
                stack.push(res);
            },
            &"/" => {
                let last = stack.pop().unwrap();
                let previous = stack.pop().unwrap();
                let res = previous / last;
                stack.push(res);
            },
            &"+" => {
                let res = stack.pop().unwrap() + stack.pop().unwrap();
                stack.push(res);
            },
            &"-" => {
                let last = stack.pop().unwrap();
                let previous = stack.pop().unwrap();
                let res = previous - last;
                stack.push(res);
            },
            &"SQRT" => {
                let last = stack.pop().unwrap();
                stack.push(last.sqrt());
            },
            &"MAX" => {
                let max_value = stack.iter().cloned().fold(0./0., f64::max);
                stack.push(max_value);
            },
            _ => {
                match op.parse::<f64>() {
                    Ok(i) => stack.push(i),
                    Err(..) => println!("this was not an integer: {}", op)
                }
            }
        }
    }

    return stack.pop().unwrap();
}

pub fn parse_split(a: &str) -> Vec<&str> {
    let vec: Vec<&str> = a.split(" ").collect();
    return vec;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    
    #[test]
    fn rpn_empty() {
        let sample = "";
        assert_eq!(0., rpn(sample));
    }

    #[test]
    fn rpn_int() {
        let sample = "42";
        assert_eq!(42., rpn(sample));
    }

    #[test]
    fn rpn_add() {
        let sample = "30 5 +";
        assert_eq!(35., rpn(sample));
    }

    #[test]
    fn rpn_minus() {
        let sample = "30 5 -";
        assert_eq!(25., rpn(sample));
    }

    #[test]
    fn rpn_multiply() {
        let sample = "30 5 *";
        assert_eq!(150., rpn(sample));
    }

    #[test]
    fn rpn_divide() {
        let sample = "30 5 /";
        assert_eq!(6., rpn(sample));
    }

    #[test]
    fn rpn_several() {
        let sample = "1 2 + 3 *";
        assert_eq!(9., rpn(sample));
    }

    #[test]
    fn rpn_several2() {
        let sample = "10 20 5 / +";
        assert_eq!(14., rpn(sample));
    }

    #[test]
    fn rpn_square_root() {
        let sample = "9 SQRT";
        assert_eq!(3., rpn(sample));
    }

    #[test]
    fn rpn_max() {
        let sample = "5 3 4 2 9 1 MAX";
        assert_eq!(9., rpn(sample));
    }

    #[test]
    fn rpn_max2() {
        let sample = "4 5 MAX 1 2 MAX *";
        assert_eq!(10., rpn(sample));
    }

    #[test]
    fn test_parse_empty() {
        let sample = "";
        let expected = vec!("");

        test_vector(expected, parse_split(sample));
    }

    #[test]
    fn test_parse_single_int() {
        let sample = "42";
        let expected = vec!("42");

        test_vector(expected, parse_split(sample));
    }

    #[test]
    fn test_parse_single_operator() {
        let sample = "*";
        let expected = vec!("*");
        
        test_vector(expected, parse_split(sample));
    }
    
    #[test]
    fn test_parse_multiple_operator() {
        let sample = "20 5 /";
        let expected = vec!("20", "5", "/");

        test_vector(expected, parse_split(sample));
    }

    fn test_vector(a: Vec<&str>, b: Vec<&str>)
    {
        assert_eq!(a.len(), b.len());

        for i in 0..=a.len()-1 {
            assert_eq!(a[i], b[i]);
        }
    }
}