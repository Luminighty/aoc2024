use super::{error::AdventError, warehouse::{Action, Robot, Tile, Warehouse}};


fn parse_warehouse(warehouse: &str) -> Result<(Warehouse, Robot), AdventError> {
  let height = warehouse.lines().count();
  let mut robot = None;
  let mut tiles = Vec::new();
  for (y, line) in warehouse.lines().enumerate() {
    for (x, c) in line.chars().enumerate() {
      match c {
        '#' => { tiles.push(Tile::WALL); },
        'O' => { tiles.push(Tile::BOX); },
        '.' => { tiles.push(Tile::FLOOR); }
        '@' => { 
          tiles.push(Tile::FLOOR);
          robot = Some((x, y));
        },
        x => return Err(AdventError::UnknownTile(x)),
      }
    }
  };
  let width = tiles.len() / height;
  let (x, y) = robot.ok_or(AdventError::RobotNotFound)?;
  let robot = Robot::new(x as i32, y as i32);
  Ok((Warehouse::new(tiles, height, width), robot))
}

fn parse_action(action: char) -> Result<Action, AdventError> {
  match action {
    '^' => Ok(Action::Up),
    '>' => Ok(Action::Right),
    '<' => Ok(Action::Left),
    'v' => Ok(Action::Down),
    _ => Err(AdventError::UnknownAction(action))
  }
}

fn parse_actions(actions: &str) -> Result<Vec<Action>, AdventError> {
  actions
    .lines()
    .flat_map(str::chars)
    .map(parse_action)
    .collect()
}

pub fn get_input() -> Result<(Warehouse, Robot, Vec<Action>), AdventError> {
  let content = unwrap!(
    std::fs::read_to_string("inp/15.txt"),
    AdventError::FileReadError
  )?;
  let (warehouse, actions) = content.split_once("\n\n").ok_or(AdventError::FileFormatError)?;
  let (warehouse, robot) = parse_warehouse(warehouse)?;
  let actions = parse_actions(actions)?;
  Ok((warehouse, robot, actions))
}
