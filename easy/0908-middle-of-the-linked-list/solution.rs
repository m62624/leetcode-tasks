// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
   pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if let Some(head) = head {
            let (mut mid, mut end) = (head.clone(), head);

            while mid.next.is_some() && end.next.is_some() {
                if let Some(mid_first_step) = mid.next {
                    mid = mid_first_step;
                }
                if let Some(end_first_step) = end.next {
                    end = end_first_step;
                    if let Some(end_second_step) = end.next {
                        end = end_second_step;
                    }
                }
            }
            return Some(mid);
        }
        None
    }
}
