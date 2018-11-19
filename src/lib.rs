pub struct ChainIntoIter<A, B>
    where
        A: IntoIterator + Copy,
        B: IntoIterator<Item = A::Item> + Copy,
{
    a: A,
    b: B,
}

pub trait Chain
    where Self: IntoIterator + Copy
{
    fn chain<B>(self, b: B) -> ChainIntoIter<Self, B>
        where B: IntoIterator<Item = Self::Item> + Copy;
}

impl<T> Chain for T
    where T: IntoIterator + Copy
{
    fn chain<B>(self, b: B) -> ChainIntoIter<Self, B>
        where B: IntoIterator<Item = Self::Item> + Copy
    {
        ChainIntoIter { a: self, b: b }
    }
}

impl<'t, A, B> IntoIterator for &'t ChainIntoIter<A, B>
where
    A: IntoIterator + Copy,
    B: IntoIterator<Item = A::Item> + Copy,
{
    type Item = A::Item;
    type IntoIter = std::iter::Chain<A::IntoIter, B::IntoIter>;
    fn into_iter(self) -> Self::IntoIter {
        self.a.into_iter().chain(self.b.into_iter())
    }
}

impl <A, B> IntoIterator for ChainIntoIter<A, B>
where
    A: IntoIterator + Copy,
    B: IntoIterator<Item = A::Item> + Copy,
{
    type Item = A::Item;
    type IntoIter = std::iter::Chain<A::IntoIter, B::IntoIter>;
    fn into_iter(self) -> Self::IntoIter {
        (&self).into_iter()
    }
}

impl<A, B> Copy for ChainIntoIter<A, B>
where
    A: IntoIterator + Copy,
    B: IntoIterator<Item = A::Item> + Copy,
{
}

impl<A, B> Clone for ChainIntoIter<A, B>
where
    A: IntoIterator + Copy,
    B: IntoIterator<Item = A::Item> + Copy,
{
    fn clone(&self) -> Self {
        ChainIntoIter { a: self.a, b: self.b }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn copy_test() {
        let result = [1, 2, 3].chain(&[4, 5, 6]);
        let x = result.chain(&[7, 8, 9]);
        let v: Vec<isize> = x.into_iter().map(|x| *x).collect();
        assert_eq!(vec!(1, 2, 3, 4, 5, 6, 7, 8, 9), v);
    }
    #[test]
    fn ref_test() {
        let result = [1, 2, 3].chain(&[4, 5, 6]);
        let x = &result.chain(&[7, 8, 9]);
        let v: Vec<isize> = x.into_iter().map(|x| *x).collect();
        assert_eq!(vec!(1, 2, 3, 4, 5, 6, 7, 8, 9), v);
    }
}
