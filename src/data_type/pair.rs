use std::fmt::Display;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

///同じ型をもつ2つの値を表す．
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pair<T> {
    pub x: T,
    pub y: T,
}

impl<T> Pair<T> {
    /// ペアを作成する．
    /// # Examples
    /// ```
    /// use data_structure::Pair;
    ///
    /// let p = Pair::new(1, 2);
    /// assert_eq!(1, p.x);
    /// assert_eq!(2, p.y);
    /// ```
    pub const fn new(x: T, y: T) -> Pair<T> {
        Pair { x, y }
    }

    /// ペアの要素を交換したものを返す．
    /// # Examples
    /// ```
    /// use data_structure::Pair;
    ///
    /// let p = Pair::new(1, 2).swap();
    /// assert_eq!(2, p.x);
    /// assert_eq!(1, p.y);
    /// ```
    pub fn swap(self) -> Pair<T> {
        Pair::new(self.y, self.x)
    }

    /// 指定した型にキャストする．
    /// # Examples
    /// ```
    /// use data_structure::Pair;
    ///
    /// let p = Pair::new(1, 2).into::<f64>();
    /// assert_eq!(1.0, p.x);
    /// assert_eq!(2.0, p.y);
    /// ```
    pub fn into<U>(self) -> Pair<U>
    where
        T: Into<U>,
    {
        Pair::new(self.x.into(), self.y.into())
    }

    /// 指定した型へのキャストを試みる．
    /// # Examples
    ///
    /// ```
    /// //Successful into
    /// use data_structure::Pair;
    ///
    /// let p = Pair::<i32>::new(1, 2).try_into::<u8>();
    /// assert_eq!(Ok(Pair::new(1, 2)), p);
    /// ```
    ///
    /// ```
    /// //Failure due to overflow
    /// use data_structure::Pair;
    ///
    /// let p = Pair::<i32>::new(1000, 2).try_into::<u8>();
    /// assert!(p.is_err());
    /// ```
    pub fn try_into<U>(self) -> Result<Pair<U>, T::Error>
    where
        T: std::convert::TryInto<U>,
    {
        Ok(Pair::new(self.x.try_into()?, self.y.try_into()?))
    }

    /// このペアの各要素に指定した操作を適用した結果をペアとして返す．
    /// # Examples
    /// ```
    /// use data_structure::Pair;
    ///
    /// let p = Pair::new(2, 5);
    /// let mapped = p.map(|e| e * e);
    /// assert_eq!(Pair::new(4, 25), mapped);
    /// ```
    pub fn map<U, F>(self, f: F) -> Pair<U>
    where
        F: Fn(T) -> U,
    {
        Pair::new(f(self.x), f(self.y))
    }
}

impl<T: Display> Display for Pair<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<T: Default> Default for Pair<T> {
    fn default() -> Self {
        Pair::new(T::default(), T::default())
    }
}

impl<T: Add> Add for Pair<T> {
    type Output = Pair<T::Output>;
    fn add(self, rhs: Self) -> Self::Output {
        Pair::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T: Sub> Sub for Pair<T> {
    type Output = Pair<T::Output>;
    fn sub(self, rhs: Self) -> Self::Output {
        Pair::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T, U> Mul<U> for Pair<T>
where
    T: Mul<U>,
    U: Copy,
{
    type Output = Pair<T::Output>;
    fn mul(self, rhs: U) -> Self::Output {
        let x = self.x * rhs;
        let y = self.y * rhs;
        Pair::new(x, y)
    }
}

impl<T, U> Div<U> for Pair<T>
where
    T: Div<U>,
    U: Copy,
{
    type Output = Pair<T::Output>;
    fn div(self, rhs: U) -> Self::Output {
        let x = self.x / rhs;
        let y = self.y / rhs;
        Pair::new(x, y)
    }
}

impl<T: AddAssign> AddAssign for Pair<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: SubAssign> SubAssign for Pair<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: MulAssign + Copy> MulAssign<T> for Pair<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<T: DivAssign + Copy> DivAssign<T> for Pair<T> {
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl<T: Neg> Neg for Pair<T> {
    type Output = Pair<T::Output>;
    fn neg(self) -> Self::Output {
        Pair::new(-self.x, -self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::Pair;

    #[test]
    fn test_new() {
        let p = Pair::new(1, 2);
        assert_eq!(1, p.x);
        assert_eq!(2, p.y);
    }

    #[test]
    fn test_into() {
        let p = Pair::<i32>::new(1, 2).into::<f64>();
        assert_eq!(1.0, p.x);
        assert_eq!(2.0, p.y);
    }

    #[test]
    fn test_try_into_success() {
        let p = Pair::<i32>::new(1, 2).try_into::<usize>();
        assert_eq!(Ok(Pair::new(1, 2)), p);
    }

    #[test]
    fn test_try_into_fail() {
        let p = Pair::new(1000, 2).try_into::<u8>();
        assert!(p.is_err());
    }

    #[test]
    fn test_map() {
        let p = Pair::new(2, 5);
        let mapped = p.map(|e| e * e);
        assert_eq!(Pair::new(4, 25), mapped);
    }

    #[test]
    fn test_default() {
        let p = Pair::new(i32::default(), i32::default());
        assert_eq!(i32::default(), p.x);
        assert_eq!(i32::default(), p.y);
    }

    #[test]
    fn test_add() {
        let lhs = Pair::new(1, 2);
        let rhs = Pair::new(3, 4);
        assert_eq!(Pair::new(4, 6), lhs + rhs);
    }

    #[test]
    fn test_sub() {
        let lhs = Pair::new(1, 2);
        let rhs = Pair::new(3, 5);
        assert_eq!(Pair::new(-2, -3), lhs - rhs);
    }

    #[test]
    fn test_mul() {
        let lhs = Pair::new(1, 2);
        let rhs = 3;
        assert_eq!(Pair::new(3, 6), lhs * rhs);
    }

    #[test]
    fn test_div() {
        let lhs = Pair::new(2, 6);
        let rhs = 2;
        assert_eq!(Pair::new(1, 3), lhs / rhs);
    }

    #[test]
    fn test_add_assign() {
        let mut lhs = Pair::new(1, 2);
        let rhs = Pair::new(3, 4);
        lhs += rhs;
        assert_eq!(Pair::new(4, 6), lhs);
    }

    #[test]
    fn test_sub_assign() {
        let mut lhs = Pair::new(1, 2);
        let rhs = Pair::new(3, 5);
        lhs -= rhs;
        assert_eq!(Pair::new(-2, -3), lhs);
    }

    #[test]
    fn test_mul_assign() {
        let mut lhs = Pair::new(1, 2);
        lhs *= 2;
        assert_eq!(Pair::new(2, 4), lhs);
    }

    #[test]
    fn test_div_assign() {
        let mut lhs = Pair::new(2, 6);
        lhs /= 2;
        assert_eq!(Pair::new(1, 3), lhs);
    }

    #[test]
    fn test_neg() {
        let p = Pair::new(1, -2);
        assert_eq!(Pair::new(-1, 2), -p);
    }
}
