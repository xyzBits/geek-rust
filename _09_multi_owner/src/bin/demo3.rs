use std::rc::Rc;

/// 假设 Node 就只包含 id 和指向 下游的指针，因为 DAG 中的一个节点可能
/// 被多个其他节点访问，所以我们使用 Rc<Node> 来表述它
/// 一个节点可能没有下游节点，所以我们用 Option<Rc<Node>> 来表述它
///
///
#[derive(Debug)]
struct Node {
    id: usize,
    downstream: Option<Rc<Node>>,

}


impl Node {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            downstream: None,
        }
    }

    pub fn update_downstream(&mut self, downstream: Rc<Node>) {
        self.downstream = Some(downstream);
    }

    pub fn get_downstream(&self) -> Option<Rc<Node>> {
        self.downstream.as_ref().map(|v| v.clone())
    }

}

fn main() {

    let mut node1 = Node::new(1);
    let mut node2 = Node::new(2);
    let mut node3 = Node::new(3);

    let node4 = Node::new(4);

    node3.update_downstream(Rc::new(node4));

    node1.update_downstream(Rc::new(node3));
    node2.update_downstream(node1.get_downstream().unwrap());

    println!("{:?}", node1);
    println!("{:?}", node2);

    let node5 = Node::new(5);

    let node3 = node1.get_downstream().unwrap();

    // node3.update_downstream(Rc::new(node5));

}