use std::ops::Deref;
pub struct BinarySearchTree<T>
where
    T:Ord,
{
    value: Option<T>,
    left: Option<Box<BinarySearchTree<T>>>,
    right: Option<Box<BinarySearchTree<T>>>,
}
impl<T> Default for BinarySearchTree<T>
where
    T:Ord,
{
    fn default() -> self {
        self::new()
    }
}

impl<T> BinarySearchTree<T>

struct BinarySearchTreeTier<'a, T>
where 
    T: Ord,
{
    stack: Vec<&'a BinarySearchTree<T>>,
}
