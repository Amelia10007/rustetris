use num_traits::{One, Signed, Zero};

/// `0`から始まり，絶対値の小さい値から交互に値を列挙していくイテレータ．
///
/// # Panics
/// `Iterator::next()`で列挙中に，`T`が表現できる値を超えるとpanicする可能性がある．
///
/// # Example
/// ```
/// assert_eq!(vec![0, 1, -1, 2, -2], Shake::new().take(5).collect::<Vec<_>>());
/// ```
#[derive(Debug)]
pub struct Shake<T> {
    current: T,
}

impl<T: Zero> Shake<T> {
    pub fn new() -> Shake<T> {
        Self { current: T::zero() }
    }
}

impl<T> Iterator for Shake<T>
where
    T: Copy + Signed + Zero + One,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let next = if self.current.is_zero() {
            T::one()
        } else if self.current.is_positive() {
            -self.current
        } else {
            debug_assert!(self.current.is_negative());
            -self.current + T::one()
        };

        let temp = self.current;
        self.current = next;
        Some(temp)
    }
}

#[cfg(test)]
mod tests_shake {
    use super::*;

    #[test]
    fn test_iter() {
        assert_eq!(
            vec![0, 1, -1, 2, -2],
            Shake::new().take(5).collect::<Vec<_>>()
        );
    }
}
