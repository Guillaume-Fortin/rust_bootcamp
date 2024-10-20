macro_rules! vec2 {
    () => {
        Vec::new()
    };
    ($($a:expr),+ $(,)?) => {
        {
            let mut res = Vec::new();
            $( res.push($a); )+
            res
        }
    };
    ($m:expr; $n:expr) => {
        {
            let mut res = Vec::new();
            for _ in 0..$n {
                res.push($m.clone());
            }
            res
        }
    };
}

#[cfg(test)]
mod tests {

    #[test]
    fn creates_empty_vector() {
        let first: Vec<i32> = vec![];
        let second: Vec<i32> = vec2![];
        assert_eq!(first, second);
    }

    #[test]
    fn creates_vector_from_list() {
        assert_eq!(vec![1, 2, 3,], vec2![1, 2, 3,]);
        assert_eq!(vec!['a', 'b', 'b', 'a'], vec2!['a', 'b', 'b', 'a']);
    }

    #[test]
    fn creates_vector_with_repeating_element() {
        assert_eq!(vec![5; 10], vec2![5;10]);
    }
}
