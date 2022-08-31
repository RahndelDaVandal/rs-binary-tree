use tree::Node;

fn main() {
    let tree = Node::from(&[1,2,3,4]);
    println!("{}", tree.get_value());
    println!("{}", tree.get_left());
    println!("{}", tree.get_right());
    print!("\n\n\n");
    println!("{:#?}", tree);
}
