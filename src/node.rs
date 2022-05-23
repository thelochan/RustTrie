use std::collections::VecDeque;
use std::fmt::Display;

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
                    cur.children.insert(i, Node::with_key(c));
                    cur = &mut cur.children[i];
                }
            }
        }
                cur.count += 1;
                cur.value.replace(s.to_string());

    }

    pub fn exists(&self, s: &str) -> bool {
        let mut cur= &self.root;
        for c in s.chars() {
            match cur.children.binary_search_by(|f| f.key.cmp(&Some(c))) {

                Ok(i) => {
                    cur = &cur.children[i];
                },
                Err(_) => {
                    return false;
                }
            }
        }
            cur.count > 0
    }



    pub fn search(&self, s: &str) -> Vec<String> { 
        let mut cur = &self.root;
        for c in s.chars() {
            match cur.children.binary_search_by(|f| f.key.cmp(&Some(c))) {

                Ok(i) => {
                    cur = &cur.children[i];
                },
                Err(_) => {
                    return Vec::new();
        }
     }
    }

      let mut results = Vec::new();
      let mut q = Vec::new();
      q.push(cur);
      while let Some(c) = q.pop() {
          for child in c.children.iter() {
              q.push(&child);
          }
          if c.count > 0 {
              let value = c.value.as_ref().unwrap();
              let count = c.count;
              results.push((count, value));
          }
      }

      results.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(b.1)));
      results.iter().map(|m| m.1.clone()).collect()
    }
}

impl Display for Trie{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut q: VecDeque<&Node> = VecDeque::new();
        let root = &self.root;
        q.push_back(root);

        while !q.is_empty () {
            for _ in 0..q.len() {
                if let Some(node) = q.pop_front() {
                    for c in node.children.iter() {
                        let r = write!(f,"{}", &c.key.unwrap());
                        if r.is_err() {
                            return r;
                        }

                        if c.children.len() > 0 {
                            q.push_back(&c);
                        }
                    }
                }

                    if q.len() > 0 {
                        let r = writeln!(f);
                        if r.is_err() {
                            return r;
                        }
                    }

            }
        }
        Ok(())
    }
}