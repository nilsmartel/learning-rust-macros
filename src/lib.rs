pub trait Number {
    fn value() -> i64;
}


macro_rules! new_number {
    ($struct_name:ident, $value:tt) => {
        struct $struct_name;

        impl Number for $struct_name {
            fn value() -> i64 {
                $value
            }
        }
    }
}

new_number!{ One, 1 }
new_number!{ Two, 2 }
new_number!{ Three, 3 }
new_number!{ Four, 4 }
new_number!{ Five, 5 }
new_number!{ Six, 6 }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn everything_works() {
        assert_eq!( One::value(), 1 );
        assert_eq!( Two::value(), 2 );
        assert_eq!( Three::value(), 3 );
        assert_eq!( Four::value(), 4 );
        assert_eq!( Five::value(), 5 );
        assert_eq!( Six::value(), 6 );

    }
}
