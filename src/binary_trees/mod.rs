struct Node {
    parent : Option<usize>,
    left : Option<usize>,
    right : Option<usize>,
    key : i32,
    position : usize,
}

pub struct BinaryTree {
    nodes : Vec<Node>,
    head : Option<usize>
}

impl Node {
    pub fn new(key : i32, pos : usize) -> Self {
        Node{parent : None, left : None, right : None, key, position: pos}
    }
}

impl BinaryTree {

    pub fn new() -> Self {
        BinaryTree{nodes : Vec::new(), head : None}
    }

    pub fn insert_key(&mut self, key : i32) {
        let mut node = Node::new(key, self.nodes.len());
        let node_pos = self.nodes.len();
        let nodes = &mut self.nodes;
        match self.head {
            Some(head) => {
                let mut x = &mut nodes[head];
                loop {
                    if key < x.key {
                        match x.left {
                            Some(left) => x = &mut nodes[left],
                            None=> break
                        }
                    } else {
                        match x.right {
                            Some(right) => x = &mut nodes[right],
                            None=> break
                        }
                    }
                }
                node.parent = Some(x.position);
                if key < x.key {
                    x.left = Some(node_pos)
                } else {
                    x.right = Some(node_pos)
                }
            },
            None => {
                self.head = Some(node_pos);
            }
        }
        nodes.push(node)
    }
}