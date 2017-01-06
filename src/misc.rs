pub fn range<'a, T: Ord>(ref data: &'a Vec<T>) -> (Option<&T>, Option<&T>) {
    (data.iter().min(), data.iter().max())
}