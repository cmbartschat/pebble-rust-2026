// use core::cell::RefCell;

// use alloc::{
//     rc::{Rc, Weak},
//     vec::Vec,
// };

// struct Node<'a> {
//     id: u32,
//     data: &'a str,
//     parent: Option<Weak<RefCell<Node<'a>>>>,
//     children: Vec<NodeRef>,
// }

// struct NodeRef {
//     handle: Rc<RefCell<Node>>,
// }

// impl Node {
//     pub fn new(id: u32) -> Self {
//         Self {
//             id,
//             parent: None,
//             children: Vec::new(),
//         }
//     }

//     pub fn set_data(&mut self, data: &'data str) {
//         self.data = data;
//     }

//     pub fn add_child(&mut self, child: NodeRef) {
//         self.children.push(child)
//         child.handle.borrow_mut().parent =
//     }

//     pub fn remove_from_parent(&mut self) {
//         let Some(parent_weak) = &self.parent else {
//             return;
//         };

//         let Some(parent_rc) = parent_weak.upgrade() else {
//             return;
//         };

//         let mut parent = parent_rc.borrow_mut();
//         let Some(child_index) = parent
//             .children
//             .iter()
//             .position(|e| e.handle.as_ptr() == self)
//         else {
//             return;
//         };

//         parent.children.swap_remove(child_index);
//     }
// }
