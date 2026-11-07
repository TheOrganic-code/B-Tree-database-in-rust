//initialization of macros  :>
const ORDER: usize=3;
const MAX_KEYS: usize=2*ORDER-1;
const MIN_KEYS: usize=ORDER-1;
//NodeId and NULL stuff
type NodeId=usize;
const NULL: NodeId=usize::MAX

//struct stuff :p
struct Node{
    keys: Vec<u64>;
    values: Vec<u64?;
    children: Vec<NodeId>,
    is_leaf: bool,
    next: NodeId,
}
//impl for node!!
impl Node{
    fn new_leaf()->Self{
        Node{
            keys:Vec::new(),
            values:Vec::new(),
            children:Vec::newW(),
            is_leaf:true,
            next:NULL,
        }
    }
}