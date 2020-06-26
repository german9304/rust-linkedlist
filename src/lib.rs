mod node;

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn create_list() {
        let mut li = LinkedList::new();
        li.insert(String::from("30"));
        li.insert(String::from("34"));
        li.insert(String::from("100"));

        assert!(li.items() == 3);
    }

    #[test]
    fn expect_head_to_not_be_none() {
        let mut li = LinkedList::new();
        li.insert(String::from("one"));
        li.insert(String::from("two"));
        let one = String::from("two");

        if let Some(head) = li.head() {
            assert_eq!(head.item(), &one);
        }
    }

}

pub struct LinkedList {
    head: Option<Box<node::Node>>,
    items: u32,
}

pub struct LinkedListIterator<'a> {
    head: &'a Option<Box<node::Node>>,
}

impl<'a> LinkedListIterator<'a> {
    pub fn new(head: &Option<Box<node::Node>>) -> LinkedListIterator {
        LinkedListIterator {
            head,
        }
    }
}


impl<'a> Iterator for LinkedListIterator<'a> {

    type Item = &'a String;
    fn next(&mut self) -> Option<Self::Item> {
        match self.head {
            Some(current_item) => {
                self.head = current_item.next();
                Some(current_item.item())
            },
            None => {
                None
            }
        }
    }
}


impl LinkedList {
    pub fn new() -> LinkedList {
        LinkedList {
            head: None,
            items: 0,
        }
    }

    pub fn iter(&self) -> LinkedListIterator {
       LinkedListIterator::new(&self.head) 
    }


    pub fn items(&self) -> u32 {
        self.items
    }

    pub fn head(&self) -> &Option<Box<node::Node>> {
        &self.head
    }

    pub fn insert(&mut self, item: String) {
        let mut n = node::Node::new(item);

        if let None = self.head {
            self.head = Some(Box::new(n));
            self.items = self.items + 1;
        } else {
            // takes value from some
            let current_head = self.head.take().unwrap_or_else(|| panic!("error"));

            n.set_next(current_head);
            self.head = Some(Box::new(n));
            self.items = self.items + 1;
        }
    }

    pub fn print_items(&self) {
        let iter = self.iter();

        for item in iter {
            println!("{}", item);
        }

    }
}
