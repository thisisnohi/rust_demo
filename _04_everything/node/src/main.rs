use std::fmt::Debug;

fn main() {
    println!("Hello, world!");

    let mut node1 = Node {
        pointer: 1,
        next: None,
    };
    node1.push(2);
    node1.push(3);

    println!("node1 = {:?}", node1);
    println!("node1 = {}", node1.pointer);
    node1.print();
}

#[derive(Debug)]
struct Node<T> {
    // 当前数据
    pointer: T,
    // 下一节点
    next: Option<Box<Node<T>>>,
}

impl<T: Debug> Node<T> {
    fn push(&mut self, data: T) {
        match &mut self.next {
            None => {
                self.next = Some(Box::new(Node {
                    pointer: data,
                    next: None,
                }))
            }
            Some(node) => node.push(data),
        }
    }

    fn print(&self) {
        println!("{:?}", self.pointer);
        match &self.next {
            None => {}
            Some(node) => {
                node.print();
            }
        }
    }
}
