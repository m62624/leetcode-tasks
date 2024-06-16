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

impl ListNode {

    fn reverse(&self) -> ListNode {
        let mut head = ListNode::new(self.val);
        let mut current = self.next.as_ref();
        while let Some(node) = current {
            head = ListNode {
                val: node.val,
                next: Some(Box::new(head)),
            };
            current = node.next.as_ref();
        }
        head
    }
}


impl std::ops::Add for ListNode {
    type Output = ListNode;

    fn add(mut self, mut other: ListNode) -> ListNode {
        let mut plus_one = false;
        let mut value = |a: &i32, b: &i32| -> i32 {
            let sum = if plus_one {
                plus_one = false;
                a + b + 1
            } else {
                a + b
            };
            if sum >= 10 {
                plus_one = true;
                sum - 10
            } else {
                sum
            }
        };

        let mut head = ListNode::new(value(&self.val, &other.val));

        loop {
            match self.next {
                Some(next) => {
                    self = *next;

                    match other.next {
                        Some(other_next) => {
                            other = *other_next;
                        }
                        None => {
                            other = ListNode::new(0);
                        }
                    }
                    head = ListNode {
                        val: value(&self.val, &other.val),
                        next: Some(Box::new(head)),
                    };
                }
                None => {
                    /*
                    L1 and l2 may have different lengths, but since we know who is shorter, 
                    we will only check `self` (L1), if L1 is shorter than L2, we swap them around
                     */

                    if other.next.is_some() {
                        let temp = self;
                        self = other;
                        other = temp;
                        continue;
                    }

                    if plus_one {
                        head = ListNode {
                            val: 1,
                            next: Some(Box::new(head)),
                        };
                    }

                    head = head.reverse();
                    break head;
                }
            }
        }
    }
}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if let Some(l1) = l1 {
            if let Some(l2) = l2 {
                return Some(Box::new(*l1 + *l2));
            }
        }
        None
    }
}
