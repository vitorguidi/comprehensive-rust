use std::{cmp::Ordering, ops::Sub};

#[derive(Debug)]
pub struct SubTree<T: Ord>(Option<Box<BTreeNode<T>>>);

#[derive(Debug)]
pub struct  BTreeNode<T: Ord> {
    val: T,
    left: SubTree<T>,
    right: SubTree<T>,
}

#[derive(Debug)]
pub struct BTree<T: Ord> {
    root: SubTree<T>
}

impl<T: Ord> SubTree<T> {
    pub fn insert(self: &mut Self, val: T) {
        match &mut self.0 {
            None => self.0 = Some(Box::new(BTreeNode { val:val, left: SubTree(None), right: SubTree(None) })),
            Some(node_box) => match val.cmp(&node_box.val) {
                Ordering::Equal => {},
                Ordering::Greater => {node_box.right.insert(val);}
                Ordering::Less => {node_box.left.insert(val);}
            }

        }
    }

    pub fn contains(self: &Self, val: T) -> bool {
        match &self.0 {
            None => false,
            Some(node_box) => match val.cmp(&node_box.val) {
                Ordering::Equal => true,
                Ordering::Greater => node_box.right.contains(val),
                Ordering::Less => node_box.left.contains(val)               
            }
        }
    }

    pub fn size(self: &Self) -> usize {
        match &self.0 {
            None => 0,
            Some(node_box) => 1 + node_box.left.size() + node_box.right.size()
        }
    }

    pub fn min(self: &Self) -> Option<&T> {
        match self {
            SubTree(None) => None,
            SubTree(Some(node)) => match &node.left {
                SubTree(None) => Some(&node.val),
                left_node => left_node.min()
            }
        }
    }

    pub fn max(self: &Self) -> Option<&T> {
        match self {
            SubTree(None) => None,
            SubTree(Some(node)) => match &node.right {
                SubTree(None) => Some(&node.val),
                right_node => right_node.max()
            }
        }
    }

    pub fn is_bst(self: &Self) -> bool {
        match self {
            SubTree(None) => true,
            SubTree(Some(node)) => {
                if let Some(max_left) = node.left.max() && *max_left > node.val {
                    return false
                }
                if let Some(min_right) = node.right.min() && *min_right < node.val {
                    return false
                }
                node.left.is_bst() && node.right.is_bst()
            }
        }
    }

}

impl<T> BTree<T> where T:Ord {
    pub fn new() -> BTree<T> {
       BTree { root: SubTree(None) }
    }

    pub fn insert(self: &mut Self, val: T) {
        self.root.insert(val)
    }

    pub fn contains(self: &Self, val: T) -> bool {
        self.root.contains(val)
    }

    pub fn size(self: &Self) -> usize {
        self.root.size()
    }

    pub fn min(self: &Self) -> Option<&T> {
        self.root.min()
    }

    pub fn max(self: &Self) -> Option<&T> {
        self.root.max()
    }

    pub fn is_bst(self: &Self) -> bool {
        self.root.is_bst()
    }
}

