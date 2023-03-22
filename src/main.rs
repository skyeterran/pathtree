use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Node<T: std::fmt::Debug> {
    Value(T),
    Tree(HashMap<String, Node<T>>),
}

impl<T: std::fmt::Debug> Node<T> {
    fn find(&self, path: &str) -> Option<&Node<T>> {
        let (head, tail) = match path.split_once('/') {
            Some(pair) => pair,
            None => match path {
                "" => { return None; },
                _ => (path, "")
            }
        };

        match head {
            "" => match tail {
                "" => Some(self),
                _ => self.find(tail)
            },
            _ => match self {
                Node::Value(v) => None,
                Node::Tree(c) => match c.get(head) {
                    Some(v) => match tail {
                        "" => Some(v),
                        _ => v.find(tail)
                    },
                    None => None
                }
            }
        }
    }
    fn add(&mut self, path: &str) {
        todo!()
    }
}

fn main() {
    let mut root: Node<String> = Node::Tree(
        HashMap::from([
            (format!("user"), Node::Tree(HashMap::from([
                (format!("name"), Node::Value(format!("Skye"))),
                (format!("age"), Node::Value(format!("26"))),
                (format!("species"), Node::Value(format!("Golden Retriever"))),
                (format!("stats"), Node::Tree(HashMap::from([
                    (format!("str"), Node::Value(format!("9"))),
                    (format!("dex"), Node::Value(format!("4"))),
                    (format!("cha"), Node::Value(format!("69"))),
                ]))),
            ]))),
        ])
    );

    let key = format!("user/stats/cha");
    println!("{key}: {:#?}", root.find(&key));
}
