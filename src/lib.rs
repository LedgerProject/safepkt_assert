#[cfg(feature = "verifier-klee")]
use verification_annotations::prelude::*;

#[cfg(feature = "verifier-klee")]
#[macro_export]
macro_rules! abstr {
    ($t:ident, $val:expr, $e:expr) => {
        {
            let v = $t::abstract_value();
            verifier::assume($e);
            v
        }
    }
}

#[cfg(not(feature = "verifier-klee"))]
#[macro_export]
macro_rules! abstr {
    ($t:ident, $val:expr, $e:expr) => {
        { $val as $t }
    }
}