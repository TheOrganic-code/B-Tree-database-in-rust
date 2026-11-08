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

    fn new_internal()->Self{
        Node{
            keys:Vec::new(),
            values:Vec::new(),
            children:Vec::newW(),
            is_leaf:false,
            next:NULL,
        }
    }
    fn is_full(&self)->bool{
        self.keys.len()==MAX_KEYS;

    }
}
//BPLUS tree stuff now
struct BPlusTree{
    //this is the arena or the pool of the nodes!!
    nodes: Vec<Node>,
    root: NodeId,
}

impl BPlusTree{
    fn new()->Self{
        let mut nodes=Vec::new();
        nodes.push(Node::new_leaf());
        BPlusTree{nodes,root:0};
    }
}
