pub struct Chain<A, B>
where
    A: IntoIterator + Copy,
    B: IntoIterator<Item = A::Item> + Copy,
{
    a: A,
    b: B,
}

impl<A, B> Chain<A, B>
where
    A: IntoIterator + Copy,
    B: IntoIterator<Item = A::Item> + Copy,
{
    pub fn new(a: A, b: B) -> Self {
        Self { a: a, b: b }
    }
}

impl<'t, A, B> IntoIterator for &'t Chain<A, B>
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

impl <A, B> IntoIterator for Chain<A, B>
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

impl<A, B> Copy for Chain<A, B>
where
    A: IntoIterator + Copy,
    B: IntoIterator<Item = A::Item> + Copy,
{
}

impl<A, B> Clone for Chain<A, B>
where
    A: IntoIterator + Copy,
    B: IntoIterator<Item = A::Item> + Copy,
{
    fn clone(&self) -> Self {
        Self { a: self.a, b: self.b }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn copy_test() {
        let result = Chain::new(&[1, 2, 3], &[4, 5, 6]);
        let x = Chain::new(result, &[7, 8, 9]);
        let v: Vec<isize> = x.into_iter().map(|x| *x).collect();
        assert_eq!(vec!(1, 2, 3, 4, 5, 6, 7, 8, 9), v);
    }
    #[test]
    fn ref_test() {
        let result = &Chain::new(&[1, 2, 3], &[4, 5, 6]);
        let x = Chain::new(result, &[7, 8, 9]);
        let v: Vec<isize> = x.into_iter().map(|x| *x).collect();
        assert_eq!(vec!(1, 2, 3, 4, 5, 6, 7, 8, 9), v);
    }
}
