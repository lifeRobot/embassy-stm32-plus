/// match Option <br />
/// if is Some, will be return value <br />
/// if is Nome, will be return custom value and exit method
#[macro_export]
macro_rules! match_some_return {
    ($opt:expr) => {
        $crate::match_some_return!($opt,())
    };
    ($opt:expr,$r:expr) => {
        $crate::match_some_exec!($opt,{return $r})
    };
}

/// match Option <br />
/// if is Some, will be return value and keep loop <br />
/// if is Nome, will be continue the loop
#[macro_export]
macro_rules! match_some_continue {
    ($opt:expr) => {
        $crate::match_some_exec!($opt,continue);
    };
}

/// match Option <br />
/// if is Some, will be return value and keep loop <br />
/// if is Nome, will be return custom value and break loop<br />
/// need to be used in loop to break value, invalid in for and while
#[macro_export]
macro_rules! match_some_break {
    ($opt:expr) => {
        $crate::match_some_break!($opt,())
    };
    ($opt:expr,$r:expr) => {
        $crate::match_some_exec!($opt,{break $r})
    };
}

/// match Option <br />
/// if is Some, will be return value <br />
/// if is Nome, will be return custom value
#[macro_export]
macro_rules! match_some_exec {
    ($opt:expr,$exec:expr) => {
        match $opt {
            Some(s) => s,
            None => $exec
        }
    };
}
