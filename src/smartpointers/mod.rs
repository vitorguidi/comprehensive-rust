#[derive(Debug)]
pub enum List<T> {
    /// A non-empty list: first element and the rest of the list.
    Element(T, Box<List<T>>),
    /// An empty list.
    Nil,
}

impl<T: Clone> List<T> {
    pub fn append(self: Self, val: T) -> List<T> {
        List::Element(val, Box::new(self))
    }
    pub fn new() -> List<T> {
        List::Nil
    }
}