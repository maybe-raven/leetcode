#[macro_export]
macro_rules! return_if_some {
    ( $v:ident ) => {
        if $v.is_some() {
            return $v;
        }
    };
    ( $e:expr ) => {{
        let return_if_some_result = $e;
        if return_if_some_result.is_some() {
            return return_if_some_result;
        }
    }};
}

#[macro_export]
macro_rules! return_value_if_some {
    ( $e:expr ) => {{
        if let Some(return_value_if_some_result) = $e {
            return return_value_if_some_result;
        }
    }};
}

#[cfg(test)]
mod tests {
    // use super::*;

    fn f_return_if_some_with_value<T>(input: Option<T>, defaut: T) -> Option<T> {
        return_if_some!(input);

        Some(defaut)
    }

    fn f_return_if_some_with_expression<T>(f: impl Fn() -> Option<T>, defaut: T) -> Option<T> {
        return_if_some! { f() }

        Some(defaut)
    }

    #[test]
    fn test_return_if_some_with_value() {
        assert_eq!(f_return_if_some_with_value(Some(42), 69), Some(42));
        assert_eq!(f_return_if_some_with_value(None, 69), Some(69));
    }

    #[test]
    fn test_return_if_some_with_expression() {
        assert_eq!(f_return_if_some_with_expression(|| Some(42), 69), Some(42));
        assert_eq!(f_return_if_some_with_expression(|| None, 69), Some(69));
    }
}
