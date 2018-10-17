pub struct Chain<A, B, I> 
where 
    A: IntoIterator<Item = I> + Copy,
    B: IntoIterator<Item = I> + Copy
{
    pub a: A,
    pub b: B,
}

pub fn chain<A, B, I>(a: A, b: B) -> Chain<A, B, I> 
where 
    A: IntoIterator<Item = I> + Copy,
    B: IntoIterator<Item = I> + Copy
{
    Chain { a: a, b: b }
}

impl<A, B, I> IntoIterator for Chain<A, B, I> 
where 
    A: IntoIterator<Item = I> + Copy,
    B: IntoIterator<Item = I> + Copy 
{
    type Item = I;
    type IntoIter = std::iter::Chain<A::IntoIter, B::IntoIter>;
    fn into_iter(self) -> Self::IntoIter {
        self.a.into_iter().chain(self.b.into_iter())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = chain(&[1, 2, 3], &[4, 5, 6]);
        let v: Vec<isize> = result.into_iter().map(|x| *x).collect();
        assert_eq!(vec!(1, 2, 3, 4, 5, 6), v);
    }
}
