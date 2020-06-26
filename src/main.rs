
fn main() {
   let items = vec![
       String::from("run"),
       String::from("walk"),
       String::from("read"),
       String::from("eat"),
   ];

   let mut li = linked_list::LinkedList::new();
   
   // insert elements into linked list
   for item in items {
       li.insert(item);
    }

   for item in li.iter() {
       println!("{}", item);
   }

}

