use iced::window::Id;
use std::collections::BTreeMap;

// Create Tables
pub fn create_tables() {
  let connection = sqlite::open("./data.db").unwrap();
  let query = "
      CREATE TABLE IF NOT EXISTS nodes (node_id INT, node_text TEXT, node_color INT, node_x INT, node_y INT, 
  ";
  connection.execute(query).unwrap();
}

struct Node {
    x: i64,
    y: i64,
    color: String,
    text: String,
}

// Pull all on-screen nodes and return them as a BTreeMap
pub fn load_nodes(window: Window, x: i64, y: i64) {
  let connection = sqlite::open("./data.db").unwrap();
  let mut node_map = BTreeMap::new();
  let query = "SELECT * FROM nodes WHERE ? < node_x < ?, ? < node_y < ?";
  for row in connection
    .prepare(query)
    .unwrap()
    .into_iter()
    .bind(x, x + size(window.x), y, y + size(window.y))
    .unwrap()
    .map(|row| row.unwrap())
  {
    let node = Node {
      x: node_x,
      y: node_y,
      color: node_color,
      text: node_text,
    };
    node_map.insert(node_id, node);
  }
  node_map
}
