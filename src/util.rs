
pub static DEBUG:bool = true;



macro_rules! Debug {
    ( $( $x:expr),*) => {
        if crate::util::DEBUG  {
            println!($($x),*);
        }
    }
}

