#[macro_export]
macro_rules! info {
    ( $( $x:expr )? ) => {
        println!("[\x1b[92mINFO\x1b[0m] {}", $( $x )?);
    };
}

#[macro_export]
macro_rules! warn {
    ( $( $x:expr )? ) => {
        println!("[\x1b[91mWARN\x1b[0m] {}", $( $x )?);
    };
}

#[macro_export]
macro_rules! error {
    ( $( $x:expr )? ) => {
        println!("[\x1b[93mERROR\x1b[0m] {}", $( $x )?);
    };
}

#[macro_export]
macro_rules! debug {
    ( $( $x:expr )? ) => {
        println!("[\x1b[92m+\x1b[0m] {}", $( $x )?);
    };
}
