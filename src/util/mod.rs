#[macro_export]
macro_rules! assert_some {
    ($reference:ident = $field:expr, $code:block) => (
        match $field {
            Some(ref $reference) => { $code },
            None => panic!("error: field {:?} is None.", stringify!($field)),
        }
    );
}

#[macro_export]
macro_rules! assert_some_mut {
    ($reference:ident = $field:expr, $code:block) => (
        match $field {
            Some(ref mut $reference) => { $code },
            None => panic!("error: field {:?} is None.", stringify!($field)),
        }
    );
}

#[macro_export]
macro_rules! match_bitmask {
    ($bits:expr, $($mask:expr => $blk:block), +) => ({
        $(if ($bits & $mask) != 0 || ($bits == $mask)  { $blk })+
    });
}
