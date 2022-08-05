pub type Span = std::ops::Range<usize>;

#[derive(Clone, Debug)]
pub struct Spanned<T> {
    pub x: T, 
    pub span: Span
}
