struct LinkedList {
    val: i32,
    next: Option<Box<LinkedList>>
}

impl LinkedList{
    fn new(val:i32) -> Self {
        LinkedList{
            val,
            next: None
        }
    }

    fn append(&mut self, val: i32){

        let mut current_node = self;

        while let Some(ref mut next_node) = current_node.next {
            current_node = next_node;
        }

        current_node.next = Some(Box::new(LinkedList::new(val)));

    }

    fn chain_and_get_head(arr: &[i32]) -> Option<LinkedList>{
        if arr.is_empty(){
            return None;
        }
        let mut head = LinkedList::new(arr[0]);
        for &val in arr.iter().skip(1){
            head.append(val);
        }

        Some(head)
    }

    fn peek(&self) -> i32 {
        let mut current_node = self;
        while let Some(ref next_node) = &current_node.next {
            current_node = next_node;
        }
        current_node.val
    }

    fn pop(&mut self) -> i32 {
        let mut current_node = self;
        //is the linked list alone? if yes, zero it out
        if current_node.next.is_none() {
            let ret = current_node.val;
            current_node.val = 0;
            return ret;
        }
        while current_node.next.as_ref().unwrap().next.is_some() {
            current_node = current_node.next.as_mut().unwrap()
        }
        current_node.next.take().unwrap().val
    }

    fn print(&self) {
        let mut current_node = self;
        while let Some(ref next_node) = &current_node.next {
            print!("{} -> ", current_node.val);
            current_node = next_node;
        }
        println!("{}", current_node.val);
    }
}

fn merge_two_lists(list1: Option<Box<LinkedList>>, list2: Option<Box<LinkedList>>) -> Option<Box<LinkedList>> {
        let mut dummy = LinkedList::new(0);
        let mut current = &mut dummy;

        let mut mut_list1 = list1;
        let mut mut_list2 = list2;

        while mut_list1.is_some() && mut_list2.is_some() {
            if mut_list1.as_ref().unwrap().val < mut_list2.as_ref().unwrap().val {
                current.next = mut_list1;
                current = current.next.as_mut().unwrap();
                mut_list1 = current.next.take(); 
            } else {
                current.next = mut_list2;
                current = current.next.as_mut().unwrap();
                mut_list2 = current.next.take();
            }
        }

        if mut_list1.is_some() {
            current.next = mut_list1;
        } else if mut_list2.is_some() {
            current.next = mut_list2;
        }

        dummy.next
    }

fn main() {
    let mut list1 = LinkedList::chain_and_get_head(&[1,3,5]).unwrap();
    let mut list2 = LinkedList::chain_and_get_head(&[2,4,6]).unwrap();
    let result = merge_two_lists(Some(Box::new(list1)), Some(Box::new(list2)));
    result.unwrap().print();
}
