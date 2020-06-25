#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_node() {
        let value = String::from("one");
        let mut node = Node::new(value);
        let next_node = Box::new(Node::new(String::from("two")));
        assert_eq!(node.item(), "one");

        node.set_next(next_node);

        if let Some(node) = node.next() {
            println!("{} ", node.item());
        }
    }
}


pub struct Node {
    item: String,
    next: Option<Box<Node>>,
}

impl Node {
    pub fn new(item: String) -> Node {
        Node { item, next: None }
    }

    pub fn set_next(&mut self, new_node: Box<Node>) {
        self.next = Some(new_node);
    }

    pub fn item(&self) -> &String {
        &self.item
    }

    pub fn next(&self) -> &Option<Box<Node>> {
        &self.next
    }
}
