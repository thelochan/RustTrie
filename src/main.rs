use node::Trie;
mod node;



fn main() {
    let mut trie = Trie::new();
    trie.insert("21e8");

    assert!(trie.exists("21e8"));
    
    println!("{}", trie);

    assert_eq!(trie.search("21", vec!["21","21e8"]))

    trie.insert("21e8")

}
