#[cfg(feature = "verifier-klee")]
#[macro_export]
macro_rules! symbolic_num {
    ($min:expr, $max:expr, $default:expr) => {
        {
            let v = verification_annotations::verifier::AbstractValue::abstract_value();
            verification_annotations::verifier::assume(v >= $min && v <= $max);
            v
        }
    };
    ($min:expr, $max:expr) => {
        symbolic_num!($min, $max, $min)
    }
}

#[cfg(feature = "verifier-klee")]
#[macro_export]
macro_rules! assert {
    ($val:expr) => {
        {
            verification_annotations::verifier::assert($val);
            assert!($val);
        }
    }
}


#[cfg(not(feature = "verifier-klee"))]
#[macro_export]
macro_rules! symbolic_num {
    ($min:expr, $max:expr, $default:expr) => {
        $default
    };
    ($min:expr, $max:expr) => {
        symbolic_num!($min, $max, $min)
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
        let x: u32 = symbolic_num!(1, 2000);
        let y: u32 = symbolic_num!(1, 2000);
        let z = x * y;
        assert!(z <= 4_000_000);
    }
}