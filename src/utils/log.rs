#[macro_export]
/// INFO macro for prettier CLI output
macro_rules! info {
    ( $( $x:expr )? ) => {
        println!("[\x1b[92mINFO\x1b[0m] {}", $( $x )?)
    };
}

#[macro_export]
/// WARN macro for prettier CLI output
macro_rules! warn {
    ( $( $x:expr )? ) => {
        println!("[\x1b[93mWARN\x1b[0m] {}", $( $x )?)
    };
}

#[macro_export]
/// ERROR macro for prettier CLI output
macro_rules! error {
    ( $( $x:expr )? ) => {
        println!("[\x1b[91mERROR\x1b[0m] {}", $( $x )?)
    };
}

#[macro_export]
/// DEBUG macro for prettier CLI output
macro_rules! debug {
    ( $( $x:expr )? ) => {
        println!("[\x1b[92m+\x1b[0m] {}", $( $x )?)
    };
}
