pub struct Chain<A, B, I>
where
    A: IntoIterator<Item = I> + Copy,
    B: IntoIterator<Item = I> + Copy,
{
    pub a: A,
    pub b: B,
}

pub fn chain<A, B, I>(a: A, b: B) -> Chain<A, B, I>
where
    A: IntoIterator<Item = I> + Copy,
    B: IntoIterator<Item = I> + Copy,
{
    Chain { a: a, b: b }
}

impl <'t, A, B, I> IntoIterator for &'t Chain<A, B, I>
where
    A: IntoIterator<Item = I> + Copy,
    B: IntoIterator<Item = I> + Copy,
{
    type Item = I;
    type IntoIter = std::iter::Chain<A::IntoIter, B::IntoIter>;
    fn into_iter(self) -> Self::IntoIter {
        self.a.into_iter().chain(self.b.into_iter())
    }
}

impl <A, B, I> IntoIterator for Chain<A, B, I>
where
    A: IntoIterator<Item = I> + Copy,
    B: IntoIterator<Item = I> + Copy,
{
    type Item = I;
    type IntoIter = std::iter::Chain<A::IntoIter, B::IntoIter>;
    fn into_iter(self) -> Self::IntoIter {
        (&self).into_iter()
    }
}

impl<A, B, I> Copy for Chain<A, B, I>
where
    A: IntoIterator<Item = I> + Copy,
    B: IntoIterator<Item = I> + Copy,
{
}

impl<A, B, I> Clone for Chain<A, B, I>
where
    A: IntoIterator<Item = I> + Copy,
    B: IntoIterator<Item = I> + Copy,
{
    fn clone(&self) -> Self {
        chain(self.a, self.b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn copy_test() {
        let result = chain(&[1, 2, 3], &[4, 5, 6]);
        let x = chain(result, &[7, 8, 9]);
        let v: Vec<isize> = x.into_iter().map(|x| *x).collect();
        assert_eq!(vec!(1, 2, 3, 4, 5, 6, 7, 8, 9), v);
    }
    #[test]
    fn ref_test() {
        let result = &chain(&[1, 2, 3], &[4, 5, 6]);
        let x = chain(result, &[7, 8, 9]);
        let v: Vec<isize> = x.into_iter().map(|x| *x).collect();
        assert_eq!(vec!(1, 2, 3, 4, 5, 6, 7, 8, 9), v);
    }
}
