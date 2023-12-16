use trie::components::{node::Node, trie::Trie};

#[test]
fn node_works() {
    let mut node = Node::new('a');
    assert_eq!(node.eow, 0);
    assert_eq!(node.children.is_empty(), true);
    assert_eq!(node.data, 'a');

    let node2 = Node::new('b');
    node.children.entry('b').or_insert(node2);
    assert_eq!(node.eow, 0);
    assert_eq!(node.children.len(), 1);
    assert_eq!(node.data, 'a');
    assert_eq!(node.children.get(&'b').unwrap().data, 'b');
}

#[test]
fn trie_works() {
    let mut trie = Trie::new();
    trie.add("test");

    assert!(trie.find("test"));
    assert_eq!(trie.size, 5);
}

#[test]
fn nested_trie_works() {
    let mut trie = Trie::new();
    trie.add("test");
    trie.add("testing");

    assert!(trie.find("test"));
    assert!(trie.find("testing"));
    assert_eq!(trie.size, 8);
}

#[test]
fn trie_length_tests() {
    let mut trie = Trie::new();
    trie.add("a");
    trie.add("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");


    assert!(trie.find("a"));
    assert!(trie.find("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"));
}

#[test]
fn completely_different_words() {
    let mut trie = Trie::new();
    trie.add("beautiful");
    trie.add("so");

    assert!(trie.find("so"));
    assert!(trie.find("beautiful"));
    assert_eq!(trie.size, 12);
}

#[test]
fn capitalization() {
    let mut trie = Trie::new();
    trie.add("tHiS");
    
    assert!(trie.find("this"));
}

#[test]
fn deletion_works() {
    todo!();
}

#[test]
fn delete_doesnt_delete_nested() {
    todo!();
}

#[test]
fn add_only_alphabet() {
    let mut trie = Trie::new();
    trie.add("this.");
    trie.add("!is");
    trie.add("n?ot");
    trie.add("punctua$tioned");

    assert!(trie.find("this"));
    assert!(trie.find("is"));
    assert!(trie.find("n"));
    assert!(trie.find("ot"));
    assert!(trie.find("punctua"));
    assert!(trie.find("tioned"));
}

#[test]
fn find_works_with_full_word() {
    let mut trie = Trie::new();
    trie.add("test");
    assert!(!trie.find("tes"));
    assert!(trie.find("test"));
    assert!(!trie.find("teste"));
}

#[test]
fn adding_same_word_twice() {
    let mut trie = Trie::new();
    trie.add("test");
    trie.add("test");

    assert!(trie.find("test"));
    assert_eq!(trie.find_amount("test"), 2);
}

#[test]
fn does_it_remove_whitespace() {
    let mut trie = Trie::new();
    trie.add("this is a test");

    assert!(trie.find("this"));
    assert!(trie.find("is"));
    assert!(trie.find("a"));
    assert!(trie.find("test"));
}

#[test]
fn standard_sentance_test() {
    let mut trie = Trie::new();
    trie.add("The quick brown fox jumped over the lazy dog.");

    assert!(trie.find("the"));
    assert!(trie.find("quick"));
    assert!(trie.find("brown"));
    assert!(trie.find("fox"));
    assert!(trie.find("jumped"));
    assert!(trie.find("over"));
    assert_eq!(trie.find_amount("the"), 2);
    assert!(trie.find("lazy"));
    assert!(trie.find("dog"));
}

#[test]
fn create_tree_from_empty_string() {
    let mut trie = Trie::new();
    trie.add("");

    assert!(!trie.find(""));
}

#[test]
fn testing_single_quote_possession() {
    let mut trie = Trie::new();
    trie.add("Rick's stuff");

    assert!(trie.find("ricks"));
    assert!(trie.find("stuff"));
}