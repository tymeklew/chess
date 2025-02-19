/*use crate::moves::Move;

pub struct Node {
    children: Vec<Box<Node>>,
    value: (i32, Option<Box<dyn Move>>),
}

impl Node {
    pub fn new(val: i32, mv: Option<Box<dyn Move>>) -> Node {
        Node {
            children: Vec::new(),
            value: (val, mv),
        }
    }

    pub fn get_current_move(&self) -> Option<Box<dyn Move>> {
        self.value.1.as_ref()
    }

    pub fn insert(&mut self, val: i32, mv: Option<Box<dyn Move>>) {
        self.children.push(Box::new(Node::new(val, mv)));
    }

    pub fn children(&self) -> &Vec<Box<Node>> {
        &self.children
    }

    pub fn add(&mut self, children: Vec<Box<Node>>) {
        self.children = children;
    }

    pub fn value(&self) -> &(i32, Option<Box<dyn Move>>) {
        &self.value
    }

    pub fn count(&self) -> usize {
        if self.children.is_empty() {
            return 1;
        }

        let mut sum = 0;
        for child in self.children.iter() {
            sum += child.count();
        }

        return sum;
    }

    pub fn display_recursive(&self, indent: usize, depth: usize) {
        // Indent to represent depth in the tree
        println!(
            "{} {} {} {} children",
            " ".repeat(indent * depth),
            self.value.0,
            match &self.value.1 {
                Some(mv) => format!("{}", mv),
                None => "Root".to_string(),
            },
            self.children.len()
        );
        if self.children.is_empty() {
            return;
        }
        for child in self.children.iter() {
            child.display_recursive(indent, depth + 1);
        }
    }
}
*/