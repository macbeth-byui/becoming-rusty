#[macro_export]
macro_rules! err {
    ($str1:expr, $str2:expr) => {
        {
            let result = format!(
                "\n=====================================================\n\
                   Error: {}\n\
                   File: {} (Line {})\n\
                   -----------------------------------------------------\n\
                   {}\n\
                   =====================================================\n",
                   $str1, file!(), line!(), $str2);
            result
        }
    };
}

pub use err;
