pub trait IterExt: Iterator {
    /// Returns the element that gives the maximum value from the
    /// specified function.
    ///
    /// If several elements are equally maximum, the first element is
    /// returned. If the iterator is empty, [`None`] is returned.
    fn first_max_by_key<F, B>(self, f: F) -> Option<Self::Item>
    where
        F: FnMut(&Self::Item) -> B,
        B: PartialOrd;
}

impl<I> IterExt for I
where
    I: Iterator,
{
    fn first_max_by_key<F, B>(self, mut f: F) -> Option<I::Item>
    where
        F: FnMut(&Self::Item) -> B,
        B: PartialOrd,
    {
        let mut max = None;

        for x in self {
            let key = f(&x);

            if let Some((max_key, _)) = &max {
                if &key > max_key {
                    max = Some((key, x));
                }
            } else {
                max = Some((key, x));
            }
        }

        max.map(|(_, val)| val)
    }
}
