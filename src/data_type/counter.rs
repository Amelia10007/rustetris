use num_traits::One;
use std::ops::{Add, RangeBounds};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Counter<T, R> {
    current: T,
    range: R,
}

impl<T, R: RangeBounds<T>> Counter<T, R>
where
    T: Copy + PartialOrd + Add<Output = T> + One,
{
    pub fn new(range: R) -> Counter<T, R> {
        use std::ops::Bound::*;

        if let Included(&start) = range.start_bound() {
            Self {
                current: start,
                range,
            }
        } else {
            panic!("start is not defined")
        }
    }

    pub fn current(&self) -> T {
        self.current
    }

    pub fn is_ended(&self) -> bool {
        let next = self.current + T::one();
        !self.range.contains(&next)
    }

    pub fn next(&mut self) -> Option<()> {
        let next = self.current + T::one();
        if self.range.contains(&next) {
            self.current = next;
            Some(())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exclusive_range() {
        let mut counter = Counter::new(1..4);
        assert_eq!(1, counter.current());
        assert!(!counter.is_ended());

        assert!(counter.next().is_some());
        assert_eq!(2, counter.current());
        assert!(!counter.is_ended());

        assert!(counter.next().is_some());
        assert_eq!(3, counter.current());
        assert!(counter.is_ended());

        assert!(counter.next().is_none());
        assert_eq!(3, counter.current());
        assert!(counter.is_ended());
    }

    #[test]
    fn test_inclusive_range() {
        let mut counter = Counter::new(1..=3);
        assert_eq!(1, counter.current());
        assert!(!counter.is_ended());

        assert!(counter.next().is_some());
        assert_eq!(2, counter.current());
        assert!(!counter.is_ended());

        assert!(counter.next().is_some());
        assert_eq!(3, counter.current());
        assert!(counter.is_ended());

        assert!(counter.next().is_none());
        assert_eq!(3, counter.current());
        assert!(counter.is_ended());
    }

    #[test]
    #[should_panic]
    fn test_unbounded_start_range() {
        let _ = Counter::new(..3);
    }

    #[test]
    #[should_panic]
    fn test_unbounded_start_range2() {
        let _ = Counter::new(..=3);
    }

    #[test]
    #[should_panic]
    fn test_unbounded_range() {
        let _ = Counter::<i32, _>::new(..);
    }
}
