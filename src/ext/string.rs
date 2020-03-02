use std::str::pattern::Pattern;

pub trait StringExtension {
    fn find_start_at<'a, P: Pattern<'a>>(&'a self, pat: P, at: usize) -> Option<usize>;
}

impl StringExtension for String {
    fn find_start_at<'a, P: Pattern<'a>>(&'a self, pat: P, at: usize) -> Option<usize> {
        self[at..].find(pat).map(|i| at + i)
    }
}
