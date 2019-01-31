pub struct ChainIntoIter<
    A: IntoIterator + Copy,
    B: IntoIterator<Item = A::Item> + Copy
> {
    a: A,
    b: B,
}

pub trait Chain: IntoIterator + Copy {
    /// ```
    /// use chain_intoiter::Chain;
    /// let result = [1, 2, 3].chain(&[4, 5, 6]);
    /// let x = result.chain(&[7, 8, 9]);
    /// let v: Vec<isize> = x.into_iter().map(|x| *x).collect();
    /// assert_eq!(vec!(1, 2, 3, 4, 5, 6, 7, 8, 9), v);
    /// ```
    fn chain<B: IntoIterator<Item = Self::Item> + Copy>(self, b: B) -> ChainIntoIter<Self, B> {
        ChainIntoIter { a: self, b: b }
    }
}

impl<T: IntoIterator + Copy> Chain for T {}

impl<
    A: IntoIterator + Copy,
    B: IntoIterator<Item = A::Item> + Copy
> IntoIterator for &'_ ChainIntoIter<A, B> {
    type Item = A::Item;
    type IntoIter = std::iter::Chain<A::IntoIter, B::IntoIter>;
    fn into_iter(self) -> Self::IntoIter {
        self.a.into_iter().chain(self.b.into_iter())
    }
}

impl<A: IntoIterator + Copy, B: IntoIterator<Item = A::Item> + Copy> Copy for ChainIntoIter<A, B> {}

impl<A: IntoIterator + Copy, B: IntoIterator<Item = A::Item> + Copy> Clone for ChainIntoIter<A, B> {
    fn clone(&self) -> Self {
        ChainIntoIter { a: self.a, b: self.b }
    }
}
