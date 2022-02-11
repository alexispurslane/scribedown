macro_rules! unwrap_ok_or_return {
    ( $e:expr ) => {
        match $e {
            Ok(x) => x,
            Err(e) => {
                println!("{:?}", e);
                return;
            }
        }
    };
}

macro_rules! unwrap_or_return {
    ( $e:expr ) => {
        match $e {
            Some(x) => x,
            None => return,
        }
    };
}

pub(crate) use unwrap_ok_or_return;
pub(crate) use unwrap_or_return;
