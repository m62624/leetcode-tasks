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
      pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        loop {
            match (list1, list2) {
                (None, None) => return None,
                (None, Some(b)) => {
                    return Some(b);
                }
                (Some(a), None) => return Some(a),
                (Some(mut a), Some(mut b)) => {
                    if a.val <= b.val {
                        a.next = Self::merge_two_lists(a.next, Some(b));
                        return Some(a);
                    } else {
                        b.next = Self::merge_two_lists(Some(a), b.next);
                        return Some(b);
                    }
                }
            }
        }
    }
}
