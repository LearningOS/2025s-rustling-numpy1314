/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/


use std::cmp::Ordering;
use std::fmt::Debug;

#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord + Debug,
{
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // 插入主方法
    fn insert(&mut self, value: T) {
        match &mut self.root {
            Some(node) => node.insert(value),
            None => self.root = Some(Box::new(TreeNode::new(value))),
        }
    }

    // 搜索主方法
    fn search(&self, value: T) -> bool {
        self.root.as_ref().map_or(false, |node| node.search(value))
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // 递归插入实现
    fn insert(&mut self, value: T) {
        match value.cmp(&self.value) {
            Ordering::Less => {
                if let Some(left) = &mut self.left {
                    left.insert(value);
                } else {
                    self.left = Some(Box::new(TreeNode::new(value)));
                }
            }
            Ordering::Greater => {
                if let Some(right) = &mut self.right {
                    right.insert(value);
                } else {
                    self.right = Some(Box::new(TreeNode::new(value)));
                }
            }
            Ordering::Equal => {} // 忽略重复值
        }
    }

    // 递归搜索实现
    fn search(&self, value: T) -> bool {
        match value.cmp(&self.value) {
            Ordering::Less => self.left.as_ref().map_or(false, |l| l.search(value)),
            Ordering::Greater => self.right.as_ref().map_or(false, |r| r.search(value)),
            Ordering::Equal => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();
        assert!(!bst.search(5));

        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        assert!(bst.search(5));
        assert!(bst.search(3));
        assert!(bst.search(7));
        assert!(bst.search(2));
        assert!(bst.search(4));

        assert!(!bst.search(1));
        assert!(!bst.search(6));
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        bst.insert(1);
        bst.insert(1);

        assert!(bst.search(1));

        if let Some(ref root) = bst.root {
            assert!(root.left.is_none());
            assert!(root.right.is_none());
        } else {
            panic!("Root should not be None");
        }
    }
}