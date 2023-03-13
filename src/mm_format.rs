use std::fmt::Display;

pub trait MMFormat {
    fn mm_format(&self, string: &mut String);
}

macro_rules! push_fmt {
    ($string:expr, $($arg:tt)*) => {
        $string.push_str(&format!($($arg)*))
    };
}

fn elem_mm_format<T: MMFormat>(elem: &T, string: &mut String, start: &mut bool) {
    if *start {
        *start = false;
    }
    else {
        push_fmt!(string, ",");
    }
    elem.mm_format(string);
}

impl<T: MMFormat> MMFormat for Vec<T> {

    fn mm_format(&self, string: &mut String) {
        push_fmt!(string, "{{");
        let mut start = true;
        for x in self.iter() {
            elem_mm_format(x, string, &mut start);
        }
        push_fmt!(string, "}}");
    }
}

macro_rules! tuple_impl_mm_format {
    ($($name:ident)+) => {
        #[allow(non_snake_case)]
        impl<$($name: MMFormat),+> MMFormat for ($($name,)+) {
            fn mm_format(&self, string: &mut String) {
                push_fmt!(string, "{{");
                let ($($name,)+) = self;
                let mut start = true;
                ($(elem_mm_format($name, string, &mut start),)+);
                push_fmt!(string, "}}");
            }
        }
    };
}

tuple_impl_mm_format!(T1);
tuple_impl_mm_format!(T1 T2);
tuple_impl_mm_format!(T1 T2 T3);
tuple_impl_mm_format!(T1 T2 T3 T4);

trait Scalar {}

impl Scalar for i8 {}
impl Scalar for i16 {}
impl Scalar for i32 {}
impl Scalar for i64 {}
impl Scalar for i128 {}
impl Scalar for isize {}

impl Scalar for u8 {}
impl Scalar for u16 {}
impl Scalar for u32 {}
impl Scalar for u64 {}
impl Scalar for u128 {}
impl Scalar for usize {}

impl Scalar for f32 {}
impl Scalar for f64 {}

impl Scalar for bool {}
impl Scalar for char {}

impl<T: Display + Scalar> MMFormat for T {
    fn mm_format(&self, string: &mut String) {
        push_fmt!(string, "{}", *self);
    }
}
