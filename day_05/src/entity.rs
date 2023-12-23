/// `IntervalTree` is a structure that represents an interval tree.
///
/// # Parameters
///
/// `root`: `Option<Box<IntervalNode>>` - A link to the root node of the interval tree. This field will be `None` if the interval tree is empty.
pub struct IntervalTree {
    root: Option<Box<IntervalNode>>,
}

/// `IntervalNode` is a structure that represents a node in an interval tree.
///
/// # Parameters
///
/// `start`: `u32` - The starting value of the interval. This field is inclusive, that means the value itself is part of the interval represented by the node.
///
/// `end`: `u32` - The ending value of the interval. Similar to `start`, this field is inclusive meaning the value itself is also part of the interval.
///
/// `left`: `Option<Box<IntervalNode>>` - A link to the left child of the current node. This field will be `None` if the node does not have a left child.
///
/// `right`: `Option<Box<IntervalNode>>` - A link to the right child of the current node. This field will be `None` if the node does not have a right child.
///
/// `shift`: `i64`  - The stored value within the node. This represents the number to shift by if it falls into this interval.
pub struct IntervalNode {
    start: u32,
    end: u32,
    left: Option<Box<IntervalNode>>,
    right: Option<Box<IntervalNode>>,
    shift: i64,
}

impl IntervalNode {
    pub fn new(start: u32, end: u32, shift: i64) -> Self {
        Self {
            start,
            end,
            shift,
            left: None,
            right: None,
        }
    }

    fn subtree(&mut self, node: &Box<IntervalNode>) -> &mut Option<Box<IntervalNode>> {
        if node.start < self.start {
            &mut self.left
        } else {
            &mut self.right
        }
    }

    fn insert(&mut self, node: Box<IntervalNode>) {
        let subtree = self.subtree(&node);
        match subtree {
            None => {
                *subtree = Some(node);
            }
            Some(ref mut child) => {
                child.insert(node);
            }
        }
    }

    fn overlaps(&self, mut start: u32, mut end: u32, overlaps: &mut Vec<(u32, u32)>) {
        if start < self.start {
            let end = end.min(self.start - 1);
            if let Some(node) = &self.left {
                node.overlaps(start, end, overlaps);
            } else {
                overlaps.push((start, end));
            }
        }
        if end > self.end {
            let start = start.max(self.end + 1);
            if let Some(node) = &self.right {
                node.overlaps(start, end, overlaps);
            } else {
                overlaps.push((start, end));
            }
        }
        if start <= self.end && end >= self.start {
            start = start.max(self.start);
            end = end.min(self.end);
            overlaps.push((self.do_shift(start), self.do_shift(end)));
        }
    }

    fn do_shift(&self, idx: u32) -> u32 {
        (idx as i64 + self.shift) as u32
    }
}

impl IntervalTree {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn insert(&mut self, start: u32, end: u32, shift: i64) {
        let node = IntervalNode::new(start, end, shift);
        match self.root {
            None => {
                self.root = Some(Box::new(node));
            }
            Some(ref mut root) => {
                root.insert(Box::new(node));
            }
        }
    }

    pub fn translate(&self, idx: u32) -> u32 {
        let mut current = &self.root;
        while let Some(node) = current {
            if idx < node.start {
                current = &node.left;
            } else if idx > node.end {
                current = &node.right;
            } else {
                return node.do_shift(idx);
            }
        }

        idx
    }

    pub fn overlaps(&self, start: u32, end: u32) -> Vec<(u32, u32)> {
        let mut overlaps = Vec::new();
        if let Some(root) = &self.root {
            root.overlaps(start, end, &mut overlaps);
        }

        overlaps
    }
}

pub struct Almanac {
    pub seeds: Vec<u32>,
    pub interval_trees: Vec<IntervalTree>,
}
