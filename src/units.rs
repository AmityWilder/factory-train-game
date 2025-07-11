/// Length
#[derive(Debug)]
pub struct Meters<T: Copy>(pub T);

impl<T: Copy> Clone for Meters<T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: Copy> Copy for Meters<T> {}

impl<T: Copy + PartialEq> PartialEq for Meters<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T: Copy + Eq> Eq for Meters<T> {}

impl<T: Copy + PartialOrd> PartialOrd for Meters<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<T: Copy + Ord> Ord for Meters<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl<T: Copy + std::hash::Hash> std::hash::Hash for Meters<T> {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T: Copy + std::ops::Neg<Output: Copy>> std::ops::Neg for Meters<T> {
    type Output = Meters<T::Output>;

    #[inline]
    fn neg(self) -> Self::Output {
        Meters(-self.0)
    }
}

impl<T: Copy + std::ops::Add<U, Output: Copy>, U: Copy> std::ops::Add<Meters<U>> for Meters<T> {
    type Output = Meters<T::Output>;

    #[inline]
    fn add(self, rhs: Meters<U>) -> Self::Output {
        Meters(self.0 + rhs.0)
    }
}

impl<T: Copy + std::ops::Sub<U, Output: Copy>, U: Copy> std::ops::Sub<Meters<U>> for Meters<T> {
    type Output = Meters<T::Output>;

    #[inline]
    fn sub(self, rhs: Meters<U>) -> Self::Output {
        Meters(self.0 - rhs.0)
    }
}

impl<T: Copy + std::ops::Mul<U, Output: Copy>, U: Copy> std::ops::Mul<Meters<U>> for Meters<T> {
    type Output = SquareMeters<T::Output>;

    #[inline]
    fn mul(self, rhs: Meters<U>) -> Self::Output {
        SquareMeters(self.0 * rhs.0)
    }
}

impl<T: Copy + std::ops::Div<U, Output: Copy>, U: Copy> std::ops::Div<Meters<U>> for Meters<T> {
    type Output = T::Output;

    #[inline]
    fn div(self, rhs: Meters<U>) -> Self::Output {
        self.0 / rhs.0
    }
}

impl<T: Copy + std::ops::Rem<U, Output: Copy>, U: Copy> std::ops::Rem<Meters<U>> for Meters<T> {
    type Output = Meters<T::Output>;

    #[inline]
    fn rem(self, rhs: Meters<U>) -> Self::Output {
        Meters(self.0 % rhs.0)
    }
}

/// Area
pub struct SquareMeters<T: Copy>(pub T);

impl<T: Copy> Clone for SquareMeters<T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: Copy> Copy for SquareMeters<T> {}

impl<T: Copy + PartialEq> PartialEq for SquareMeters<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T: Copy + Eq> Eq for SquareMeters<T> {}

impl<T: Copy + PartialOrd> PartialOrd for SquareMeters<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<T: Copy + Ord> Ord for SquareMeters<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl<T: Copy + std::hash::Hash> std::hash::Hash for SquareMeters<T> {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T: Copy + std::ops::Neg<Output: Copy>> std::ops::Neg for SquareMeters<T> {
    type Output = SquareMeters<T::Output>;

    #[inline]
    fn neg(self) -> Self::Output {
        SquareMeters(-self.0)
    }
}

impl<T: Copy + std::ops::Add<U, Output: Copy>, U: Copy> std::ops::Add<SquareMeters<U>> for SquareMeters<T> {
    type Output = SquareMeters<T::Output>;

    #[inline]
    fn add(self, rhs: SquareMeters<U>) -> Self::Output {
        SquareMeters(self.0 + rhs.0)
    }
}

impl<T: Copy + std::ops::Sub<U, Output: Copy>, U: Copy> std::ops::Sub<SquareMeters<U>> for SquareMeters<T> {
    type Output = SquareMeters<T::Output>;

    #[inline]
    fn sub(self, rhs: SquareMeters<U>) -> Self::Output {
        SquareMeters(self.0 - rhs.0)
    }
}

impl<T: Copy + std::ops::Mul<U, Output: Copy>, U: Copy> std::ops::Mul<Meters<U>> for SquareMeters<T> {
    type Output = CubicMeters<T::Output>;

    #[inline]
    fn mul(self, rhs: Meters<U>) -> Self::Output {
        CubicMeters(self.0 * rhs.0)
    }
}

impl<T: Copy + std::ops::Mul<U, Output: Copy>, U: Copy> std::ops::Mul<SquareMeters<U>> for Meters<T> {
    type Output = CubicMeters<T::Output>;

    #[inline]
    fn mul(self, rhs: SquareMeters<U>) -> Self::Output {
        CubicMeters(self.0 * rhs.0)
    }
}

impl<T: Copy + std::ops::Div<U, Output: Copy>, U: Copy> std::ops::Div<Meters<U>> for SquareMeters<T> {
    type Output = Meters<T::Output>;

    #[inline]
    fn div(self, rhs: Meters<U>) -> Self::Output {
        Meters(self.0 / rhs.0)
    }
}

impl<T: Copy + std::ops::Div<U, Output: Copy>, U: Copy> std::ops::Div<SquareMeters<U>> for SquareMeters<T> {
    type Output = T::Output;

    #[inline]
    fn div(self, rhs: SquareMeters<U>) -> Self::Output {
        self.0 / rhs.0
    }
}

impl<T: Copy + std::ops::Rem<U, Output: Copy>, U: Copy> std::ops::Rem<SquareMeters<U>> for SquareMeters<T> {
    type Output = SquareMeters<T::Output>;

    #[inline]
    fn rem(self, rhs: SquareMeters<U>) -> Self::Output {
        SquareMeters(self.0 % rhs.0)
    }
}

/// Volume
pub struct CubicMeters<T: Copy>(pub T);

impl<T: Copy> Clone for CubicMeters<T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: Copy> Copy for CubicMeters<T> {}

impl<T: Copy + PartialEq> PartialEq for CubicMeters<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T: Copy + Eq> Eq for CubicMeters<T> {}

impl<T: Copy + PartialOrd> PartialOrd for CubicMeters<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<T: Copy + Ord> Ord for CubicMeters<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl<T: Copy + std::hash::Hash> std::hash::Hash for CubicMeters<T> {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T: Copy + std::ops::Neg<Output: Copy>> std::ops::Neg for CubicMeters<T> {
    type Output = CubicMeters<T::Output>;

    #[inline]
    fn neg(self) -> Self::Output {
        CubicMeters(-self.0)
    }
}

impl<T: Copy + std::ops::Add<U, Output: Copy>, U: Copy> std::ops::Add<CubicMeters<U>> for CubicMeters<T> {
    type Output = CubicMeters<T::Output>;

    #[inline]
    fn add(self, rhs: CubicMeters<U>) -> Self::Output {
        CubicMeters(self.0 + rhs.0)
    }
}

impl<T: Copy + std::ops::Sub<U, Output: Copy>, U: Copy> std::ops::Sub<CubicMeters<U>> for CubicMeters<T> {
    type Output = CubicMeters<T::Output>;

    #[inline]
    fn sub(self, rhs: CubicMeters<U>) -> Self::Output {
        CubicMeters(self.0 - rhs.0)
    }
}

impl<T: Copy + std::ops::Div<U, Output: Copy>, U: Copy> std::ops::Div<CubicMeters<U>> for CubicMeters<T> {
    type Output = SquareMeters<T::Output>;

    #[inline]
    fn div(self, rhs: CubicMeters<U>) -> Self::Output {
        SquareMeters(self.0 / rhs.0)
    }
}

impl<T: Copy + std::ops::Div<U, Output: Copy>, U: Copy> std::ops::Div<SquareMeters<U>> for CubicMeters<T> {
    type Output = Meters<T::Output>;

    #[inline]
    fn div(self, rhs: SquareMeters<U>) -> Self::Output {
        Meters(self.0 / rhs.0)
    }
}

impl<T: Copy + std::ops::Div<U, Output: Copy>, U: Copy> std::ops::Div<Meters<U>> for CubicMeters<T> {
    type Output = T::Output;

    #[inline]
    fn div(self, rhs: Meters<U>) -> Self::Output {
        self.0 / rhs.0
    }
}

impl<T: Copy + std::ops::Rem<U, Output: Copy>, U: Copy> std::ops::Rem<CubicMeters<U>> for CubicMeters<T> {
    type Output = CubicMeters<T::Output>;

    #[inline]
    fn rem(self, rhs: CubicMeters<U>) -> Self::Output {
        CubicMeters(self.0 % rhs.0)
    }
}

/// Per time
pub struct PerSecond<T: Copy>(T);



impl<T: Copy> Clone for PerSecond<T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: Copy> Copy for PerSecond<T> {}

impl<T: Copy + PartialEq> PartialEq for PerSecond<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T: Copy + Eq> Eq for PerSecond<T> {}

impl<T: Copy + PartialOrd> PartialOrd for PerSecond<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<T: Copy + Ord> Ord for PerSecond<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl<T: Copy + std::hash::Hash> std::hash::Hash for PerSecond<T> {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T: Copy + std::ops::Neg<Output: Copy>> std::ops::Neg for PerSecond<T> {
    type Output = PerSecond<T::Output>;

    #[inline]
    fn neg(self) -> Self::Output {
        PerSecond(-self.0)
    }
}

impl<T: Copy + std::ops::Add<U, Output: Copy>, U: Copy> std::ops::Add<PerSecond<U>> for PerSecond<T> {
    type Output = PerSecond<T::Output>;

    #[inline]
    fn add(self, rhs: PerSecond<U>) -> Self::Output {
        PerSecond(self.0 + rhs.0)
    }
}

impl<T: Copy + std::ops::Sub<U, Output: Copy>, U: Copy> std::ops::Sub<PerSecond<U>> for PerSecond<T> {
    type Output = PerSecond<T::Output>;

    #[inline]
    fn sub(self, rhs: PerSecond<U>) -> Self::Output {
        PerSecond(self.0 - rhs.0)
    }
}

impl<T: Copy + std::ops::Mul<U, Output: Copy>, U: Copy> std::ops::Mul<U> for PerSecond<T> {
    type Output = PerSecond<T::Output>;

    #[inline]
    fn mul(self, rhs: U) -> Self::Output {
        PerSecond(self.0 * rhs)
    }
}

impl<T: Copy + std::ops::Div<U, Output: Copy>, U: Copy> std::ops::Div<PerSecond<U>> for PerSecond<T> {
    type Output = T::Output;

    #[inline]
    fn div(self, rhs: PerSecond<U>) -> Self::Output {
        self.0 / rhs.0
    }
}

impl<T: Copy + std::ops::Rem<U, Output: Copy>, U: Copy> std::ops::Rem<PerSecond<U>> for PerSecond<T> {
    type Output = PerSecond<T::Output>;

    #[inline]
    fn rem(self, rhs: PerSecond<U>) -> Self::Output {
        PerSecond(self.0 % rhs.0)
    }
}
