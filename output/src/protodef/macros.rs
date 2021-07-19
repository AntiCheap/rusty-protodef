#[macro_export]
macro_rules! scope {
    ($var:ident, $val: expr, $post: expr) => {{
        #[allow(unused_variables)]
        let $var = Protodef::Void();
        #[allow(unused_mut)]
        let mut $var = $val;
        #[allow(unused_variables)]
        let here = &mut $var;
        $post;
        $var
    }};
}

#[macro_export]
macro_rules! count_all {
    ($val: expr) => {
        Some(*$val as usize)
    };
}

#[macro_export]
macro_rules! count_signed {
    ($val: expr) => {{
        if $val.is_negative() {
            None
        } else {
            Some(*$val as usize)
        }
    }};
}

#[macro_export]
macro_rules! count_float {
    ($val: expr) => {{
        let pos = $val.is_finite() && $val.is_sign_positive();
        if $val.fract() == 0.0 && pos {
            Some(*$val as usize)
        } else {
            None
        }
    }};
}

#[macro_export]
macro_rules! comparer {
    (u8, $val: expr) => {
        count_all!($val)
    };
    (u16, $val: expr) => {
        count_all!($val)
    };
    (u32, $val: expr) => {
        count_all!($val)
    };
    (u64, $val: expr) => {
        count_all!($val)
    };
    (i8, $val: expr) => {
        count_signed!($val)
    };
    (i16, $val: expr) => {
        count_signed!($val)
    };
    (i32, $val: expr) => {
        count_signed!($val)
    };
    (i64, $val: expr) => {
        count_signed!($val)
    };
    (f32, $val: expr) => {
        count_float!($val)
    };
    (f64, $val: expr) => {
        count_float!($val)
    };
}
