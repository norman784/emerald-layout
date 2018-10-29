# Emerald-layout

Emerald-layout is a flexible layout engine inspired in
[Yoga](https://github.com/facebook/yoga)

## Draft API

Port Yoga to Rust, then change whatever I don't like of way
that yoga name method/properties.

```rust
extern crate emerald_layout;

use emerald_layout::enums::{Justify, Direction, Node};

let root = Node::new();
root.set_width(500);
root.set_height(300);
root.set_justify_content(JustifyContent::Center);

let node1 = Node::new();
node1.set_width(100);
node1.set_height(100);

let node2 = Node::new();
node2.set_width(100);
node2.set_height(100);

root.insert_child(node1);
root.insert_child(node2);

root.calculate_layout(500, 300, Direction::LTR);
println!("{:?}", root.get_computed_layout());
// {left: 0, top: 0, width: 500, height: 300}
println!("{:?}", node1.get_computed_layout());
// {left: 150, top: 0, width: 100, height: 100}
println!("{:?}", node2.get_computed_layout());
// {left: 250, top: 0, width: 100, height: 100}
```
