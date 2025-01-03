#[macro_export]
#[rustfmt::skip]
macro_rules! __parse_type {
    ($var:expr => str => str) => ($var);

    ($var:expr => str => try str) => (Some($var));
    ($var:expr => str => try char) => ({
        let t = $var;
        if t.len() == 1 {
            t.chars().next()
        } else {
            None
        }
    });
    ($var:expr => str => try usize) => ($var.parse::<usize>().ok());
    ($var:expr => str => try u128) => ($var.parse::<u128>().ok());
    ($var:expr => str => try u64) => ($var.parse::<u64>().ok());
    ($var:expr => str => try u32) => ($var.parse::<u32>().ok());
    ($var:expr => str => try u16) => ($var.parse::<u16>().ok());
    ($var:expr => str => try u8) => ($var.parse::<u8>().ok());
    ($var:expr => str => try isize) => ($var.parse::<isize>().ok());
    ($var:expr => str => try i128) => ($var.parse::<i128>().ok());
    ($var:expr => str => try i64) => ($var.parse::<i64>().ok());
    ($var:expr => str => try i32) => ($var.parse::<i32>().ok());
    ($var:expr => str => try i16) => ($var.parse::<i16>().ok());
    ($var:expr => str => try i8) => ($var.parse::<i8>().ok());
    ($var:expr => str => try f64) => ($var.parse::<f64>().ok());
    ($var:expr => str => try f32) => ($var.parse::<f32>().ok());

    ($var:expr => char => char) => ($var);
    ($var:expr => char => str) => ($var.to_string());

    ($var:expr => char => try usize) => ($var.to_digit(10).map(|v| v as usize));
    ($var:expr => char => try u128) => ($var.to_digit(10).map(|v| v as u128));
    ($var:expr => char => try u64) => ($var.to_digit(10).map(|v| v as u64));
    ($var:expr => char => try u32) => ($var.to_digit(10));
    ($var:expr => char => try u16) => ($var.to_digit(10).map(|v| v as u16));
    ($var:expr => char => try u8) => ($var.to_digit(10).map(|v| v as u8));
    ($var:expr => char => try isize) => ($var.to_digit(10).map(|v| v as isize));
    ($var:expr => char => try i128) => ($var.to_digit(10).map(|v| v as i128));
    ($var:expr => char => try i64) => ($var.to_digit(10).map(|v| v as i64));
    ($var:expr => char => try i32) => ($var.to_digit(10).map(|v| v as i32));
    ($var:expr => char => try i16) => ($var.to_digit(10).map(|v| v as i16));
    ($var:expr => char => try i8) => ($var.to_digit(10).map(|v| v as i8));
    ($var:expr => char => try f64) => ($var.to_digit(10).map(|v| v as f64));
    ($var:expr => char => try f32) => ($var.to_digit(10).map(|v| v as f32));

    ($var:expr => $from:tt => $to:tt) => {
        $crate::utils::parser::__parse_type__!($var => $from => try $to).unwrap()
    };

    ($var:expr => $from:tt => try $type:tt) => ($type::try_from($var));
    ($var:expr => $from:tt => $type:tt) => ($type::from($var));
}
pub use __parse_type as __parse_type__;

/// Split a string on a separator and parse the parts into a tuple with the given types.
#[macro_export]
macro_rules! __parse {
    // Store element as identifier.
    // [
    //   $name
    //   (
    //      as $type ||
    //      with $transformer ||
    //      with [{ nested } => result]
    //   )
    // ]
    // name; $type will be &str

    ([[ $($tmpnames:ident)+ ]] $input:expr => [ $ident:ident as $type:tt ]) => {
        let $ident = $crate::utils::parser::__parse_type__!($input => str => $type);
    };
    ([[ $($tmpnames:ident)+ ]] $input:expr => [ $ident:ident with $transformer:expr ]) => {
        let $ident = $transformer($input);
    };
    ([[ $($tmpnames:ident)+ ]] $input:expr => [ $ident:ident with { $($nested:tt)+ } => $result:expr ]) => {
        let $ident = $crate::utils::parser::parse!($input => { $($nested)+ } => $result);
    };
    ([[ $($tmpnames:ident)+ ]] $input:expr => $ident:ident) => {
        $crate::utils::parser::parse!([[ $($tmpnames)* ]] $input => [ $ident as str ]);
    };

    // Split element into a collection.
    // [
    //   $name split
    //   (on $sep; default " ")
    //   (
    //      into iterator ||
    //      into ($collection);
    //      default $collection Vec
    //   )
    //   (
    //      try? as $type ||
    //      with [nested-bracketed] ||
    //      with { nested } => result ||
    //      try? with $transformer;
    //      default $type &str)
    //   )
    // ]

    // start
    ([[ $($tmpnames:ident)+ ]] $input:expr => [ $ident:ident split $($rest:tt)* ]) => {
        #[allow(unused_mut)]
        let mut $ident = $crate::utils::parser::parse!(split; $input => [ ]; $($rest)*);
    };
    // on $sep
    (split; $input:expr => [ ]; on $sep:literal $($rest:tt)*) => {
        $crate::utils::parser::parse!(split; $input => [ on $sep ]; $($rest)*)
    };
    (split; $input:expr => [ ]; $($rest:tt)*) => {
        $crate::utils::parser::parse!(split; $input => [ on " " ]; $($rest)*)
    };
    // into $collection
    (split; $input:expr => [ on $sep:literal ]; into iterator $($rest:tt)*) => {
        $crate::utils::parser::parse!(split; $input => [ on $sep into::[iter] (Iterator) ]; $($rest)*)
    };
    (split; $input:expr => [ on $sep:literal ]; into ($collection:ty) $($rest:tt)*) => {
        $crate::utils::parser::parse!(split; $input => [ on $sep into::[] ($collection) ]; $($rest)*)
    };
    (split; $input:expr => [ on $sep:literal ]; $($rest:tt)*) => {
        $crate::utils::parser::parse!(split; $input => [ on $sep into::[] (Vec<_>) ]; $($rest)*)
    };
    // (try)? as $type
    (split; $input:expr => [ on $sep:literal into::[$($iterargs:tt)*] ($collection:ty) ]; as $type:tt) => {
        $crate::utils::parser::parse!(split; $input => [ on $sep into::[$($iterargs)*] ($collection) with::[] (
            |item| $crate::utils::parser::__parse_type__!(item => str => $type)
        ) ];)
    };
    (split; $input:expr => [ on $sep:literal into::[$($iterargs:tt)*] ($collection:ty) ]; try as $type:tt) => {
        $crate::utils::parser::parse!(split; $input => [ on $sep into::[$($iterargs)*] ($collection) with::[try] (
            |item| $crate::utils::parser::__parse_type__!(item => str => try $type)
        ) ];)
    };
    // with [nested-bracketed]
    (split; $input:expr => [ on $sep:literal into::[$($iterargs:tt)*] ($collection:ty) ]; with [ $($nested:tt)+ ]) => {
        $crate::utils::parser::parse!(split; $input => [ on $sep into::[$($iterargs)*] ($collection) with::[] (
            |item| {
                $crate::utils::parser::parse!([[ tmpvar ]] item => [ result $($nested)+ ]);
                result
            }
        ) ];)
    };
    // with { nested } => result
    (split; $input:expr => [ on $sep:literal into::[$($iterargs:tt)*] ($collection:ty) ]; with { $($nested:tt)+ } => $result:expr) => {
        $crate::utils::parser::parse!(split; $input => [ on $sep into::[$($iterargs)*] ($collection) with::[]
            |item| $crate::utils::parser::parse!(item => { $($nested)+ } => $result)
        ];)
    };
    // (try)? with $transformer
    (split; $input:expr => [ on $sep:literal into::[$($iterargs:tt)*] ($collection:ty) ]; with $transformer:expr) => {
        $crate::utils::parser::parse!(split; $input => [ on $sep into::[$($iterargs)*] ($collection) with::[] ($transformer) ];)
    };
    (split; $input:expr => [ on $sep:literal into::[$($iterargs:tt)*] ($collection:ty) ]; try with $transformer:expr) => {
        $crate::utils::parser::parse!(split; $input => [ on $sep into::[$($iterargs)*] ($collection) with::[try] ($transformer) ];)
    };
    // done
    (split; $input:expr => [ on $sep:literal into::[iter] ($_:ty) $($rest:tt)* ];) => {
        $crate::utils::parser::parse!(split; $input => [[ on $sep $($rest)* ]];)
    };
    (split; $input:expr => [ on $sep:literal into::[] ($collection:ty) $($rest:tt)* ];) => {
        $crate::utils::parser::parse!(split; $input => [[ on $sep $($rest)* ]];).collect::<$collection>()
    };
    (split; $input:expr => [[ on $sep:literal ]];) => {
        $input.split($sep)
    };
    (split; $input:expr => [[ on $sep:literal with::[try] $transformer:expr ]];) => {
        $input.split($sep).filter_map($transformer)
    };
    (split; $input:expr => [[ on $sep:literal with::[] $transformer:expr ]];) => {
        $input.split($sep).map($transformer)
    };

    // Split element into a collection by chars.
    // [
    //   $name chars
    //   (
    //      into iterator ||
    //      into ($collection);
    //      default $collection Vec
    //   )
    //   (
    //      try? as $type ||
    //      try? with $transformer;
    //      default $type &str)
    //   )
    // ]

    // start
    ([[ $($tmpnames:ident)+ ]] $input:expr => [ $ident:ident chars $($rest:tt)* ]) => {
        #[allow(unused_mut)]
        let mut $ident = $crate::utils::parser::parse!(chars; $input => [ ]; $($rest)*);
    };
    // into $collection
    (chars; $input:expr => [ ]; into iterator $($rest:tt)*) => {
        $crate::utils::parser::parse!(chars; $input => [ into::[iter] (Iterator) ]; $($rest)*)
    };
    (chars; $input:expr => [ ]; into ($collection:ty) $($rest:tt)*) => {
        $crate::utils::parser::parse!(chars; $input => [ into::[] ($collection) ]; $($rest)*)
    };
    (chars; $input:expr => [ ]; $($rest:tt)*) => {
        $crate::utils::parser::parse!(chars; $input => [ into::[] (Vec<_>) ]; $($rest)*)
    };
    // (try)? as $type
    (chars; $input:expr => [ into::[$($iterargs:tt)*] ($collection:ty) ]; as $type:tt) => {
        $crate::utils::parser::parse!(chars; $input => [ into::[$($iterargs)*] ($collection) with::[] (
            |item| $crate::utils::parser::__parse_type__!(item => char => $type)
        ) ];)
    };
    (chars; $input:expr => [ into::[$($iterargs:tt)*] ($collection:ty) ]; try as $type:tt) => {
        $crate::utils::parser::parse!(chars; $input => [ into::[$($iterargs)*] ($collection) with::[try] (
            |item| $crate::utils::parser::__parse_type__!(item => char => try $type)
        ) ];)
    };
    // (try)? with $transformer
    (chars; $input:expr => [ into::[$($iterargs:tt)*] ($collection:ty) ]; with $transformer:expr) => {
        $crate::utils::parser::parse!(chars; $input => [ into::[$($iterargs)*] ($collection) with::[] ($transformer) ];)
    };
    (chars; $input:expr => [ into::[$($iterargs:tt)*] ($collection:ty) ]; try with $transformer:expr) => {
        $crate::utils::parser::parse!(chars; $input => [ into::[$($iterargs)*] ($collection) with::[try] ($transformer) ];)
    };
    // done
    (chars; $input:expr => [ into::[iter] ($_:ty) $($rest:tt)* ];) => {
        $crate::utils::parser::parse!(chars; $input => [[ $($rest)* ]];)
    };
    (chars; $input:expr => [ into::[] ($collection:ty) $($rest:tt)* ];) => {
        $crate::utils::parser::parse!(chars; $input => [[ $($rest)* ]];).collect::<$collection>()
    };
    (chars; $input:expr => [[ ]];) => {
        $input.chars()
    };
    (chars; $input:expr => [[ with::[try] $transformer:expr ]];) => {
        $input.chars().filter_map($transformer)
    };
    (chars; $input:expr => [[ with::[] $transformer:expr ]];) => {
        $input.chars().map($transformer)
    };

    // Empty tail.
    ([[ $($tmpnames:ident)+ ]] $input:expr =>) => {};

    // Ignore element.
    ([[ $($tmpnames:ident)+ ]] $input:expr => _) => {
        let _ = $input;
    };

    // Recursively process everything until the next instance of a given literal.
    ([[ $($tmpnames:ident)+ ]] $input:expr => $first:tt $sep:literal $($rest:tt)*) => {
        ::paste::paste!{
            let mut [< $($tmpnames)+ >] = $input.splitn(2, $sep);
        };
        $crate::utils::parser::parse!([[ $($tmpnames)+ _1 ]] ::paste::paste!([< $($tmpnames)+ >]).next().unwrap() => $first);
        $crate::utils::parser::parse!([[ $($tmpnames)+ _2 ]] ::paste::paste!([< $($tmpnames)+ >]).next().unwrap() => $($rest)*);
    };

    // Leading literal.
    ([[ $($tmpnames:ident)+ ]] $input:expr => $prefix:literal $($rest:tt)*) => {
        $crate::utils::parser::parse!([[ $($tmpnames)+ ]] $input.strip_prefix($prefix).unwrap() => $($rest)*);
    };

    // Entrypoint.
    ($input:expr => { $($stuff:tt)+ } => $result:expr) => {
        {
            $crate::utils::parser::parse!([[ tmpvar ]] $input => $($stuff)*);
            $result
        }
    };
    ($input:expr => $($stuff:tt)+) => {
        $crate::utils::parser::parse!([[ tmpvar ]] $input => $($stuff)*);
    };
}
pub use __parse as parse;

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn parse_singular() {
        parse!("foo" => foo);
        assert_eq!(foo, "foo");
    }

    #[test]
    fn parse_singular_type() {
        parse!("12" => [foo as u8]);
        assert_eq!(foo, 12);
    }

    #[test]
    fn parse_singular_custom() {
        parse!("Hello" => [foo with str::to_lowercase]);
        assert_eq!(foo, "hello");
    }

    #[test]
    fn parse_single_sep() {
        parse!("foo, 12" => foo ", " [bar as u8]);
        assert_eq!(foo, "foo");
        assert_eq!(bar, 12);
    }

    #[test]
    fn parse_multi_sep() {
        parse!("foo, 12, -22" => foo ", " [bar as u8] ", " [baz as i8]);
        assert_eq!(foo, "foo");
        assert_eq!(bar, 12);
        assert_eq!(baz, -22);
    }

    #[test]
    fn parse_skip() {
        parse!("foo, baz, bar" => foo ", " _ ", " bar);
        assert_eq!(foo, "foo");
        assert_eq!(bar, "bar");
    }

    #[test]
    fn parse_multi_literal() {
        parse!("foo, 12" => foo "," " " [bar as u8]);
        assert_eq!(foo, "foo");
        assert_eq!(bar, 12);
    }

    #[test]
    fn parse_leading_literal() {
        parse!("(foo, bar)" => "(" foo ", " bar);
        assert_eq!(foo, "foo");
        assert_eq!(bar, "bar)");
    }

    #[test]
    fn parse_trailing_literal() {
        parse!("(foo, bar)" => foo ", " bar ")");
        assert_eq!(foo, "(foo");
        assert_eq!(bar, "bar");
    }

    #[test]
    fn parse_surrounding_literals() {
        parse!("(foo, bar)" => "(" foo ", " bar ")");
        assert_eq!(foo, "foo");
        assert_eq!(bar, "bar");
    }

    #[test]
    fn parse_type() {
        parse!("1 2" => [foo as u8] " " [bar as usize]);
        assert_eq!(foo, 1);
        assert_eq!(bar, 2);
    }

    #[test]
    fn parse_list() {
        parse!("1 2" => [items split]);
        assert_eq!(items, vec!["1", "2"]);
    }

    #[test]
    fn parse_list_custom_sep() {
        parse!("1-2" => [items split on "-"]);
        assert_eq!(items, vec!["1", "2"]);
    }

    #[test]
    fn parse_list_custom_type() {
        parse!("1 2" => [items split as u8]);
        assert_eq!(items, vec![1, 2]);
    }

    #[test]
    fn parse_list_custom_collection() {
        parse!("1 2" => [items split into (HashSet<_>)]);
        assert_eq!(items, HashSet::from(["1", "2"]));
    }

    #[test]
    fn parse_list_iterator() {
        parse!("1 2" => [items split into iterator]);
        assert_eq!(items.next(), Some("1"));
        assert_eq!(items.next(), Some("2"));
        assert_eq!(items.next(), None);
    }

    #[test]
    fn parse_list_to_map() {
        let sub: for<'a> fn(&'a str) -> (&'a str, u8) = |pair| {
            parse!(pair => name "=" [value as u8]);
            (name, value)
        };
        parse!("a=1 b=2" => [items split on " " into (HashMap<_, _>) with sub]);
        assert_eq!(items, HashMap::from([("a", 1), ("b", 2)]));
    }

    #[test]
    fn parse_list_try_as() {
        parse!("12 angry men" => [items split try as u8]);
        assert_eq!(items, vec![12]);
    }

    #[test]
    fn parse_list_surrounding_literals() {
        parse!("(1 2)" => "(" [items split] ")");
        assert_eq!(items, vec!["1", "2"]);
    }

    #[test]
    fn parse_list_nested_list() {
        parse!("1,2 3,4" => [items split with [split on ',']]);
        assert_eq!(items, vec![vec!["1", "2"], vec!["3", "4"]]);
    }

    #[test]
    fn parse_list_nested_chars() {
        parse!("12 34" => [items split with [chars]]);
        assert_eq!(items, vec![vec!['1', '2'], vec!['3', '4']]);
    }

    #[test]
    fn parse_list_nested_with_expression() {
        parse!("a=1 b=2" => [items split with { key "=" [value as u8] } => (key, value)]);
        assert_eq!(items, vec![("a", 1), ("b", 2)]);
    }

    #[test]
    fn parse_list_nested_with_expression_nested_list() {
        parse!("1,2 3,4" => [items split with { [pair split on "," as u8] } => pair.into_iter().max().unwrap()]);
        assert_eq!(items, vec![2, 4]);
    }

    #[test]
    fn parse_chars() {
        parse!("12" => [items chars]);
        assert_eq!(items, vec!['1', '2']);
    }

    #[test]
    fn parse_chars_custom_type() {
        parse!("12" => [items chars as u8]);
        assert_eq!(items, vec![1, 2]);
    }

    #[test]
    fn parse_chars_custom_collection() {
        parse!("12" => [items chars into (HashSet<_>)]);
        assert_eq!(items, HashSet::from(['1', '2']));
    }

    #[test]
    fn parse_chars_try_as() {
        parse!("1a2b" => [items chars try as u8]);
        assert_eq!(items, vec![1, 2]);
    }

    #[test]
    fn parse_result_expression() {
        let result = parse!("foo bar" => { foo " " bar } => (foo, bar));
        assert_eq!(result, ("foo", "bar"));
    }
}
