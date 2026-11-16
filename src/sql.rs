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

// Pull all on-screen nodes and return them as a BTreeMap
pub fn load_nodes(window_id: Id, x: i64, y: i64) {
  let mut node_map = BTreeMap::new();
  let query = "SELECT * FROM nodes WHERE ? < node_x < ?, ? < node_y < ?";
  x + size(window_id.x);
  y + size(window_id.y);
  for row in connection
      .prepare(query)
      .unwrap()
      .into_iter()
      .bind((1, 50))
      .unwrap()
      .map(|row| row.unwrap())
  {
      node_map.insert();
  }
}
