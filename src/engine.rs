
pub fn parse_split(a: &str) -> Vec<&str> {
    let mut vec = Vec::with_capacity(10);
    vec.push(a);
    return vec;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_parse_empty() {
        let sample = "";
        let mut expected = Vec::with_capacity(1);
        expected.push("");

        test_vector(expected, parse_split(sample));
    }

    #[test]
    fn test_parse_single_int() {
        let sample = "42";
        let mut expected = Vec::with_capacity(1);
        expected.push("42");

        test_vector(expected, parse_split(sample));
    }

    #[test]
    fn test_parse_single_operator() {
        let sample = "*";
        let mut expected = Vec::with_capacity(1);
        expected.push("*");
        
        test_vector(expected, parse_split(sample));
    }
    
    #[test]
    fn test_parse_multiple_operator() {
        let sample = "20 5 /";
        let mut expected = Vec::with_capacity(1);
        expected.push("*");

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