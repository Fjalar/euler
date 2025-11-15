pub mod big_u {
    use std::{
        fmt,
        ops::{AddAssign, MulAssign},
    };

    pub struct BigU(Vec<u32>);

    const BIGU_ELEM_MAX: u64 = 1_000_000_000;

    impl AddAssign<u32> for BigU {
        fn add_assign(&mut self, rhs: u32) {
            let mut idx = 0;
            let mut carry = rhs;

            while let Some(elem) = self.0.get_mut(idx) {
                let potential_new = *elem as u64 + carry as u64;
                if potential_new >= BIGU_ELEM_MAX {
                    let smaller = potential_new - BIGU_ELEM_MAX;
                    *elem = smaller as u32;
                    carry = 1;
                    idx += 1;
                } else {
                    *elem = potential_new as u32;
                    carry = 0;
                    break;
                }
            }
            if carry != 0 {
                self.0.push(carry);
            }
        }
    }

    impl MulAssign<u32> for BigU {
        fn mul_assign(&mut self, rhs: u32) {
            let mut idx = 0;
            let mut carry = 0;

            while let Some(elem) = self.0.get_mut(idx) {
                let new = carry + *elem as u64 * rhs as u64;
                *elem = (new % BIGU_ELEM_MAX) as u32;
                carry = new / BIGU_ELEM_MAX;
                idx += 1;
                if idx > self.0.len() {
                    self.0.push(0);
                }
            }
            if carry != 0 {
                self.0.push(carry as u32);
            }
        }
    }

    impl BigU {
        pub fn new(init: u32) -> Self {
            let mut new = BigU(vec![0]);
            new += init;
            new
        }
    }

    impl Default for BigU {
        fn default() -> Self {
            Self::new(0)
        }
    }

    impl fmt::Display for BigU {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let length = self.0.len();
            write!(f, "{}", self.0.last().unwrap())?;
            for i in (0..length - 1).rev() {
                write!(f, "{:09}", self.0[i])?
            }
            Ok(())
        }
    }

    impl fmt::Debug for BigU {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_tuple("BigU").field(&self.0).finish()
        }
    }
}
