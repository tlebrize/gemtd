use crate::Game;
use bevy::prelude::*;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::hash::{Hash, Hasher};

#[derive(Clone, Eq, PartialEq, Hash, Copy)]
pub enum Direction {
   North,
   South,
   West,
   East,
}

impl Direction {
   fn opposite(&self) -> Self {
      match self {
         Self::North => Self::South,
         Self::South => Self::North,
         Self::East => Self::West,
         Self::West => Self::East,
      }
   }
}

pub type NodeId = u32;

pub struct Node {
   pub walkable: bool,
   x: usize,
   y: usize,
   id: NodeId,
}

impl PartialEq for Node {
   fn eq(&self, other: &Self) -> bool {
      self.id == other.id
   }
}

impl Eq for Node {}

impl Hash for Node {
   fn hash<H: Hasher>(&self, state: &mut H) {
      self.id.hash(state);
   }
}

#[derive(Default)]
pub struct Graph {
   nodes: HashMap<NodeId, Node>,
   positions: HashMap<(usize, usize), NodeId>,
   neighbors: HashMap<(Direction, NodeId), NodeId>,
   counter: NodeId,
   pub start: NodeId,
   pub end: NodeId,
}

impl Graph {
   pub fn get_neighbors(&self, id: NodeId) -> HashMap<Direction, NodeId> {
      let mut neighbors = HashMap::new();
      for direction in vec![
         Direction::North,
         Direction::South,
         Direction::East,
         Direction::West,
      ]
      .iter()
      {
         if let Some(node_id) = self.neighbors.get(&(*direction, id)) {
            neighbors.insert(*direction, *node_id);
         }
      }
      neighbors
   }

   pub fn add(&mut self, walkable: bool, x: usize, y: usize) -> NodeId {
      self.counter += 1;
      let id = self.counter;
      let mut node = Node { walkable, x, y, id };
      self.positions.insert((x, y), id);

      if x > 0 {
         if let Some(north_id) = self.positions.get(&(x - 1, y)) {
            self.neighbors.insert((Direction::North, id), *north_id);
            self.neighbors.insert((Direction::South, *north_id), id);
         }
      }

      if let Some(south_id) = self.positions.get(&(x + 1, y)) {
         self.neighbors.insert((Direction::South, id), *south_id);
         self.neighbors.insert((Direction::North, *south_id), id);
      }

      if let Some(east_id) = self.positions.get(&(x, y + 1)) {
         self.neighbors.insert((Direction::East, id), *east_id);
         self.neighbors.insert((Direction::West, *east_id), id);
      }

      if y > 0 {
         if let Some(west_id) = self.positions.get(&(x, y - 1)) {
            self.neighbors.insert((Direction::East, id), *west_id);
            self.neighbors.insert((Direction::West, *west_id), id);
         }
      }

      self.nodes.insert(id, node);
      id
   }

   pub fn set_node_walkability(&mut self, node_id: NodeId, walkable: bool) {
      let mut node = self.nodes.get_mut(&node_id).unwrap();
      node.walkable = walkable;
   }

   pub fn is_walkable(&self, node_id: NodeId) -> bool {
      if let Some(node) = self.nodes.get(&node_id) {
         node.walkable
      } else {
         false
      }
   }

   pub fn bfs(&self) -> Option<Vec<NodeId>> {
      let mut frontier = VecDeque::new();
      let mut came_from = HashMap::new();
      frontier.push_back(self.start);
      came_from.insert(self.start, None);

      while !frontier.is_empty() {
         let current = frontier.pop_front().unwrap();
         if current == self.end {
            break;
         }

         for (_, next_id) in self.get_neighbors(current).iter() {
            if self.is_walkable(*next_id) && !came_from.contains_key(next_id) {
               frontier.push_back(*next_id);
               came_from.insert(*next_id, Some(current));
            }
         }
      }

      let mut path = vec![];
      let mut current = self.end;
      while current != self.start {
         path.push(current);
         if let Some(origin) = came_from.get(&current) {
            current = origin.unwrap();
         } else {
            println!("No path");
            return None;
         }
      }
      path.push(self.start);
      Some(path)
   }

   pub fn get_node_coordinates(&self, node_id: NodeId) -> Option<(usize, usize)> {
      self.nodes.get(&node_id).map(|node| (node.x, node.y))
   }
}

fn update_path(
   mut commands: Commands,
   mut new_path: EventReader<NewPathEvent>,
   graph: Res<Graph>,
   game: Res<Game>,
   mut query: Query<&mut Sprite>,
) {
   for event in new_path.iter() {
      for (mut sprite) in query.iter_mut() {
         if sprite.color == Color::PURPLE {
            sprite.color = Color::BLACK;
         }
      }
      for node_id in event.0.iter() {
         let (x, y) = graph.get_node_coordinates(*node_id).unwrap();
         let mut sprite = query.get_mut(game.grid[y][x]).unwrap();
         sprite.color = Color::PURPLE;
      }
   }
}

pub struct NewPathEvent(pub Vec<NodeId>);

pub struct PathfindingPlugin;

impl Plugin for PathfindingPlugin {
   fn build(&self, app: &mut App) {
      app.insert_resource(Graph::default())
         .add_event::<NewPathEvent>()
         .add_system(update_path);
   }
}
