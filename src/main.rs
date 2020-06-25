fn main() {
   let mut li = linked_list::LinkedList::new();
   li.insert(String::from("one"));
   li.insert(String::from("two"));
   li.insert(String::from("three"));

   for item in li.iter() {
       println!("item {}", item);
   }

}

