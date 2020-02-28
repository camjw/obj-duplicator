pub trait StringExtension {
    pub fn find_start_at<'a, P: Pattern<'a>>(&'a self, pat: P, at: usize) -> Option<usize>
}

impl StringExtension for String {
    fn find_start_at<'a, P: Pattern<'a>>(&'a self, pat: P, at: usize) -> Option<usize> {
        slice[at..].find(pat).map(|i| at + i)
    }
}
