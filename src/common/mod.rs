pub type Span = std::ops::Range<usize>;
use std::sync::Arc;
use ariadne;

#[derive(Clone, Debug)]
pub struct Spanned<T> {
    pub source: Arc<String>,
    pub x: T, 
    pub span: Span
}
impl<T> ariadne::Span for Spanned<T> {
    type SourceId = Arc<String>;
    fn start(&self) -> usize {
        self.span.start()
        
    }
    fn end(&self) -> usize {
        self.span.end()
        
    }
    fn len(&self) -> usize {
        ExactSizeIterator::len(&self.span)
    }
    fn source(&self) -> &Self::SourceId {
        &self.source
    }
}
