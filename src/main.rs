use std::{collections::VecDeque, fmt::{Display, Debug}, io::Write, fs::File};

#[derive(Debug, Clone)]
struct Node<T>{
    val: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(val: T, left: Option<Box<Node<T>>>, right: Option<Box<Node<T>>>) -> Self {
        Node {
            val,
            left,
            right,
        }
    }
}

fn inorder<T: Copy + Debug>(root: Box<Node<T>>) -> Vec<T> {
    let mut out: Vec<T> = Vec::new();
    let mut stack: Vec<Box<Node<T>>> = Vec::new();
    let mut current = Some(root);
    loop {
        if let Some(val) = current {
            stack.push(val.clone());
            current = val.left;
        } else {
            if let Some(val) = stack.pop() {
                out.push(val.val);
                current = val.right;
            } else {
                break;
            }
        }
    }
    out
}

fn breadth_first_traversal<T: Write, A: Display>(root: Box<Node<A>>, writer: &mut T) -> std::io::Result<()> {
   let mut queue: VecDeque<Option<Box<Node<A>>>> = VecDeque::new();
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

fn write_to_dot<T: Write, A: Display>(writer: &mut T, root: Box<Node<A>>) -> std::io::Result<()> {
   write!(writer, "digraph {{\n")?;
   let mut queue: VecDeque<Box<Node<A>>> = VecDeque::new();
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
    // let mut f = File::create_new("graph.dot").unwrap();
    let p = Node::new(2,
        Some(Box::new(Node::new(4,
                None, Some(Box::new(Node::new(5, None, None)))
              ))), None);
    // write_to_dot(&mut f, Box::new(p)).unwrap();
    let a = inorder(Box::new(p));
    println!("{:?}", a);
}

//
// digraph {
//  a -> b
//  a -> c
// 
// }
