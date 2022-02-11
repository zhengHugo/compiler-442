pub struct Node<T>
where
    T: PartialEq,
{
    index: usize,
    value: T,
    parent: Option<usize>,
    children: Vec<usize>,
}

pub struct Tree<T>
where
    T: PartialEq,
{
    nodes: Vec<Node<T>>,
}

impl<T> Node<T>
where
    T: PartialEq,
{
    fn new(index: usize, value: T) -> Self {
        Self {
            index,
            value,
            parent: None,
            children: vec![],
        }
    }
}
