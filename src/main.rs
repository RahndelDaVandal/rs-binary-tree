use tree::Node;
use tree::StrBuilder;

fn main() {
    let tree = Node::from(&[1,2,3,4]);
    let mut s = StrBuilder::new();
    s.traverse_preorder(tree);
    println!("{:#?}", s);
}
