#[derive(Default)]
pub struct Node{
    pub children: Vec<Node>,
    pub key: Option<char>,
    pub value: Option<String>,
    pub count: usize,
}

impl Node {

    pub fn new() -> Self {
        Node {
            ..Default::default()
        }
    }
    pub fn with_key(c: char) -> Self {
        Node{
            key: Some(c),
            ..Default::default()
        }
    }
}

pub struct Trie {
    pub root: Node,
}

impl Trie {

    pub fn new() -> Self {
        Trie { root: Node::new() }
    }

    pub fn insert(&mut self, s: &str) {
        let mut cur = &mut self.root;
        for c in s.chars() {
            match cur.children.binary_search_by(|f| f.key.cmp(&Some(c))) {

                Ok(i) => {
                    cur =&mut cur.children[i];

                },
                Err(i) => {
                    cur.children.insert(i, Node::with_key(c: char));
                    cur = &mut cur.children[i];
                }
            }
        }


    }
    pub fn exists(&self, s: &str) -> bool {false}
    pub fn search(&self, s: &str) -> Vec<String> { Vec::new() }
}