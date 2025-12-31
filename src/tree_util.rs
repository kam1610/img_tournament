use serde::{Serialize, Deserialize};
use std::path::Path;
use std::path::PathBuf;
use std::rc::Rc;
use std::cell::Cell;
use std::cell::RefCell;

// Decision //////////////////////////////////////////////////
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Decision { Undef, Left, Right }
// Node ////////////////////////////////////////////////////
#[derive(Debug)]
pub struct Node{
    pub path  : Option<PathBuf>, // leaf->Some, branch->None
    pub left  : Option<Rc<RefCell<Node>>>,
    pub right : Option<Rc<RefCell<Node>>>,
    pub depth : usize,
    pub decision: Cell<Decision>,
    pub opt   : isize,
}
impl Node{
    pub fn set_decision(&self, d: Decision){ self.decision.set(d); }
    pub fn get_match_up_list(n: Rc<RefCell<Node>>) -> Vec<Rc<RefCell<Node>>>{
        let mut v = vec![];
        // left
        let l = n.borrow().left.clone();
        if (l.is_some()) && (l.clone().unwrap().borrow().path.is_none()) {
            v.extend( Self::get_match_up_list(l.expect("left node")) ); }
        // right
        let r = n.borrow().right.clone();
        if (r.is_some()) && (r.clone().unwrap().borrow().path.is_none()) {
            v.extend( Self::get_match_up_list(r.expect("right node")) ); }
        // self
        if n.borrow().path.is_none(){ v.push(n); }

        // todo: sort min depth -> max depth
        v.sort_by(|a, b| a.borrow().depth.cmp(&b.borrow().depth)  );

        return v;
    }
    fn to_serializable(&self) -> SerializableNode{
        SerializableNode{
            path  : self.path.clone(),
            left  : self.left.as_ref().
                map(|l| Box::new(l.borrow().to_serializable())),
            right : self.right.as_ref().
                map(|l| Box::new(l.borrow().to_serializable())),
            depth : self.depth,
            decision: Cell::new(self.decision.get()),
            opt : self.opt,
        }
    }
    fn new_leaf(path: PathBuf, opt: isize) -> Self{
        Self{
            path  : Some(path),
            left  : None,
            right : None,
            depth : 1,
            decision: Cell::new(Decision::Undef),
            opt   : opt
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
// resolve_winner_leaf /////////////////////////////////////
fn resolve_winner_leaf(node: &Rc<RefCell<Node>>) -> Option<PathBuf> {
    let n = node.borrow();
    match (&n.left, &n.right, &n.decision.get()) {
        // self is leaf
        (      _,       _,               _) if n.path.is_some() => n.path.clone(),
        // not selected yet
        (      _,       _, Decision::Undef) => None,
        // left is winner
        (Some(l),       _, Decision::Left ) => resolve_winner_leaf(l),
        // right is winner
        (      _, Some(r), Decision::Right) => resolve_winner_leaf(r),
        // unexpected case
        _ => None,
    }
}
// next candidate //////////////////////////////////////////
fn find_next_undef_node(
    node: &Rc<RefCell<Node>>,
) -> Option<(usize, Rc<RefCell<Node>>)> {
    let n = node.borrow();

    let left  = n.left.as_ref().and_then(|l| find_next_undef_node(l));
    let right = n.right.as_ref().and_then(|r| find_next_undef_node(r));

    let self_node = if (n.decision.get() == Decision::Undef) && (n.path.is_none()){
        Some((n.depth, Rc::clone(node))) }
    else {
        None };

    [self_node, left, right]
        .into_iter()
        .flatten()
        .min_by_key(|(d, _)| *d)
}
#[derive(Debug)]
pub struct NextCandidate {
    node: Rc<RefCell<Node>>,
    winner_leaf_l: Option<PathBuf>,
    winner_leaf_r: Option<PathBuf>,
}
pub fn next_candidate(root: &Rc<RefCell<Node>>) -> Option<NextCandidate> {
    let (_, node) = find_next_undef_node(root)?; // node: Rc<RefCell<Node>>

    let winner_l = resolve_winner_leaf( &node.borrow().left.clone().unwrap() );
    let winner_r = resolve_winner_leaf( &node.borrow().right.clone().unwrap());

    Some(NextCandidate {
        node,
        winner_leaf_l: winner_l,
        winner_leaf_r: winner_r,
    })
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
pub fn insert(node: Option<Rc<RefCell<Node>>>, path: PathBuf, opt: isize) -> Rc<RefCell<Node>>{
    if node.is_none(){
        return Rc::new(RefCell::new(Node::new_leaf(path, opt))); }

    let node_rc = node.unwrap();

    let n = node_rc.clone();
    let mut n = n.borrow_mut();

    if n.path.is_some(){ // leaf -> convert brach and add leaf
        let cur_leaf = Rc::new(RefCell::new(Node::new_leaf(n.path.take().unwrap(), n.opt)));
        let new_leaf = Rc::new(RefCell::new(Node::new_leaf(path, opt)));
        n.left  = Some(cur_leaf);
        n.right = Some(new_leaf);
        n.update_depth();
        return node_rc;
    }

    if Node::depth(&n.left) <= Node::depth(&n.right) {
        n.left  = Some(insert(n.left.take(), path, opt));
    } else {
        n.right = Some(insert(n.right.take(), path, opt));
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
        println!("{}Leaf: opt={}, {}", indent, n.opt, path.display());
    } else {
        println!("{}Node (opt={}, h={}, balance={})", indent, n.opt, n.depth, n.balance_factor());
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
    decision: Cell<Decision>,
    opt : isize,
}
impl SerializableNode {
    fn to_node(&self) -> Rc<RefCell<Node>>{
        let node = Rc::new(RefCell::new(Node {
            path : self.path.clone(),
            left : self.left.as_ref().map(|l| l.to_node()),
            right: self.right.as_ref().map(|l| l.to_node()),
            depth: self.depth,
            decision: self.decision.clone(),
            opt : self.opt,
        }));
        return node;
    }
}
