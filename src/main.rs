use std::{collections::VecDeque, io::Write, fs::File};

#[derive(Debug)]
struct Node {
    val: u32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    pub fn new(val: u32, left: Option<Box<Node>>, right: Option<Box<Node>>) -> Self {
        Node {
            val,
            left,
            right,
        }
    }
}

fn breadth_first_traversal<T: Write>(root: Box<Node>, writer: &mut T) -> std::io::Result<()> {
   let mut queue: VecDeque<Option<Box<Node>>> = VecDeque::new();
   let mut depth_queue: VecDeque<u32> = VecDeque::new();
   let mut current_depth = 0;
   queue.push_back(Some(root));
   depth_queue.push_back(current_depth);
   loop {
       let r = queue.pop_front();
       if let Some(n) = r {
           let new_depth = depth_queue.pop_front().unwrap();
           if  new_depth != current_depth {
               write!(writer, "\n")?;
               current_depth = new_depth;
           }
           if let Some(v) = n {
               write!(writer, "{},", v.val)?;
               queue.push_back(v.right); 
               depth_queue.push_back(current_depth + 1);
               queue.push_back(v.left);
               depth_queue.push_back(current_depth + 1);
           } else {
               write!(writer, "x,")?;
           }
       } else {
           break;
       }
   } 
   Ok(())
}

fn write_to_dot<T: Write>(writer: &mut T, root: Box<Node>) -> std::io::Result<()> {
   write!(writer, "digraph {{\n")?;
   let mut queue: VecDeque<Box<Node>> = VecDeque::new();
   queue.push_back(root);
   loop {
       let r = queue.pop_front();
       if let Some(n) = r {
           if let Some(r) = n.right {
               write!(writer, "{} -> {}\n", n.val, r.val)?;
               queue.push_back(r);
           }
           if let Some(l) = n.left {
               write!(writer, "{} -> {}\n", n.val, l.val)?;
               queue.push_back(l);
           }
       } else {
           break;
       }
   } 
   write!(writer, "}}")?;
   Ok(())
}

fn main() {
    let mut f = File::create_new("graph.dot").unwrap();
    let p = Node::new(2,
        Some(Box::new(Node::new(4,
                None, Some(Box::new(Node::new(5, None, None)))
              ))), None);
    write_to_dot(&mut f, Box::new(p)).unwrap();
}

//
// digraph {
//  a -> b
//  a -> c
// 
// }
