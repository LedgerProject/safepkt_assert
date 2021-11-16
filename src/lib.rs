#[cfg(feature = "verifier-klee")]
use verification_annotations::prelude::*;

#[cfg(feature = "verifier-klee")]
#[macro_export]
macro_rules! abstract_value {
    ($t:ident, $val:expr) => {
        {
            let v = $t::abstract_value();
            verifier::assume($e);
            v
        }
    }
}
#[cfg(feature = "verifier-klee")]
#[macro_export]
macro_rules! assume {
    ($val:expr) => {
        {
            verifier::assume($val);
        }
    }
}
#[cfg(feature = "verifier-klee")]
#[macro_export]
macro_rules! assert {
    ($val:expr) => {
        {
            verifier::assert($val);
            assert!($val);
        }
    }
}

#[cfg(not(feature = "verifier-klee"))]
#[macro_export]
macro_rules! abstract_value {
    ($t:ident, $val:expr) => {
        { $val as $t }
    }
}
#[cfg(not(feature = "verifier-klee"))]
#[macro_export]
macro_rules! assume {
    ($val:expr) => {
        ()
    }
}
#[cfg(not(feature = "verifier-klee"))]
#[macro_export]
macro_rules! assert {
    ($val:expr) => {
        {
            assert!($val);
        }
    }
}

#[cfg(tests)]
mod tests {
    #[test]
    fn try_it() {
        let x = abstract_value!(u32, 1000);
        assume!(x > 0 && x < 2000);
        let y = abstract_value!(u32, 1000);
        assume!(y > 0 && y < 2000);
        let z = x * y;
        assert!(z < 4_000_000);
    }
}