use serde::{Serialize, Deserialize};
use std::path::Path;
use std::path::PathBuf;
use std::rc::Rc;
use std::cell::RefCell;

// Decision //////////////////////////////////////////////////
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum Decision { Undef, Left, Right }
// Node ////////////////////////////////////////////////////
#[derive(Debug)]
pub struct Node{
    path  : Option<PathBuf>, // leaf->Some, branch->None
    left  : Option<Rc<RefCell<Node>>>,
    right : Option<Rc<RefCell<Node>>>,
    depth : usize,
    decision: Decision,
}
impl Node{
    fn to_serializable(&self) -> SerializableNode{
        SerializableNode{
            path  : self.path.clone(),
            left  : self.left.as_ref().
                map(|l| Box::new(l.borrow().to_serializable())),
            right : self.right.as_ref().
                map(|l| Box::new(l.borrow().to_serializable())),
            depth : self.depth,
            decision: self.decision,
        }
    }
    fn new_leaf(path: PathBuf) -> Self{
        Self{
            path  : Some(path),
            left  : None,
            right : None,
            depth : 1,
            decision: Decision::Undef
        }
    }
    fn new_branch(left: Rc<RefCell<Node>>, right: Rc<RefCell<Node>>) -> Self{
        Self{
            path  : None,
            left  : Some(left.clone()),
            right : Some(right.clone()),
            depth : 1 +
                usize::max(Self::depth(&Some(left)), Self::depth(&Some(right))),
            decision: Decision::Undef
        }
    }
    fn depth(node: &Option<Rc<RefCell<Node>>>) -> usize{
        node.as_ref().map_or(0, |n| n.borrow().depth ) }
    fn update_depth(&mut self){
        self.depth = 1 +
            usize::max(Self::depth(&self.left), Self::depth(&self.right)); }
    fn balance_factor(&self) -> isize {
        Self::depth(&self.left) as isize - Self::depth(&self.right) as isize }
}
// rotate //////////////////////////////////////////////////
fn rotate_right(n: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
    let l  = n.borrow_mut().left.take().expect("left-node doesn't exist when rotating right");
    let lr = l.borrow_mut().right.take();

    l.borrow_mut().right = Some(Rc::clone(&n));
    n.borrow_mut().left = lr;

    n.borrow_mut().update_depth();
    l.borrow_mut().update_depth();

    return l;
}
fn rotate_left(n: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
    let r  = n.borrow_mut().right.take().expect("left-node doesn't exist when rotating right");
    let rl = r.borrow_mut().left.take();

    r.borrow_mut().left  = Some(Rc::clone(&n));
    n.borrow_mut().right = rl;

    n.borrow_mut().update_depth();
    r.borrow_mut().update_depth();

    return r;
}
pub fn insert(node: Option<Rc<RefCell<Node>>>, path: PathBuf) -> Rc<RefCell<Node>>{
    if node.is_none(){
        return Rc::new(RefCell::new(Node::new_leaf(path))); }

    let node_rc = node.unwrap();

    let n = node_rc.clone();
    let mut n = n.borrow_mut();

    if n.path.is_some(){ // leaf -> convert brach and add leaf
        let cur_leaf = Rc::new(RefCell::new(Node::new_leaf(n.path.take().unwrap())));
        let new_leaf = Rc::new(RefCell::new(Node::new_leaf(path)));
        n.left  = Some(cur_leaf);
        n.right = Some(new_leaf);
        n.update_depth();
        return node_rc;
    }

    if Node::depth(&n.left) <= Node::depth(&n.right) {
        n.left  = Some(insert(n.left.take(), path));
    } else {
        n.right = Some(insert(n.right.take(), path));
    }
    n.update_depth();

    let balance = n.balance_factor();

    if balance > 1 { // right is too deep
        if n.left.as_ref().unwrap().borrow().balance_factor() < 0 {
            let left = n.left.take().unwrap();
            n.left = Some(rotate_left(left));
        }
        return rotate_right(Rc::clone(&node_rc));
    }

    if balance < -1 { // left is too deep
        if n.right.as_ref().unwrap().borrow().balance_factor() > 0 {
            let right = n.right.take().unwrap();
            n.right = Some(rotate_right(right));
        }
        return rotate_left(Rc::clone(&node_rc));
    }

    // already balanced
    return node_rc;
}
// print ///////////////////////////////////////////////////
pub fn print_tree(node: &Rc<RefCell<Node>>, depth: usize){
    let indent = "  ".repeat(depth);
    let n = node.borrow();
    if let Some(path) = &n.path{
        println!("{}Leaf: {}", indent, path.display());
    } else {
        println!("{}Node (h={}, balance={})", indent, n.depth, n.balance_factor());
        if let Some(left) = &n.left {
            print_tree(left, depth + 1);
        }
        if let Some(right) = &n.right {
            print_tree(right, depth + 1);
        }
    }
}
// SerializableNode ////////////////////////////////////////
#[derive(Serialize, Deserialize, Debug)]
struct SerializableNode{
    path : Option<PathBuf>,
    left : Option<Box<SerializableNode>>,
    right: Option<Box<SerializableNode>>,
    depth: usize,
    decision: Decision,
}
impl SerializableNode {
    fn to_node(&self) -> Rc<RefCell<Node>>{
        let node = Rc::new(RefCell::new(Node {
            path : self.path.clone(),
            left : self.left.as_ref().map(|l| l.to_node()),
            right: self.right.as_ref().map(|l| l.to_node()),
            depth: self.depth,
            decision: self.decision,
        }));
        return node;
    }
}
