struct Solution;
use crate::util::*;

trait Leaves {
    fn dfs(&self, leaves: &mut Vec<i32>);
}

impl Leaves for TreeLink {
    fn dfs(&self, leaves: &mut Vec<i32>) {
        if let Some(node) = self {
            let node = node.borrow();
            let left = &node.left;
            let right = &node.right;
            if left.is_none() && right.is_none() {
                leaves.push(node.val);
            } else {
                Self::dfs(left, leaves);
                Self::dfs(right, leaves);
            }
        }
    }
}

impl Solution {
    fn leaf_similar(root1: TreeLink, root2: TreeLink) -> bool {
        let mut leaves1: Vec<i32> = vec![];
        let mut leaves2: Vec<i32> = vec![];
        root1.dfs(&mut leaves1);
        root2.dfs(&mut leaves2);
        leaves1 == leaves2
    }
}

#[test]
fn test() {
    let root1 = tree!(
        3,
        tree!(5, tree!(6), tree!(2, tree!(7), tree!(4))),
        tree!(1, tree!(9), tree!(8))
    );
    let root2 = tree!(
        4,
        tree!(5, tree!(6), tree!(2, tree!(7), tree!(4))),
        tree!(1, tree!(9), tree!(8))
    );
    assert_eq!(Solution::leaf_similar(root1, root2), true);
}
