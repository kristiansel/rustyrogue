extern crate tcod;

use std::string::String;
use tcod::{Console, RootConsole, BackgroundFlag};
use tcod::input::Key;
use tcod::input::KeyCode::{Up, Down, Left, Right, Escape};

struct Vec2<T> {
    x: T,
    y: T,
}

struct Creature {
    name: String,
    pos: Vec2<i32>,
}

#[derive(Copy, Clone, Debug)]
struct Tile {
    solid: bool,
    opaque: bool,
}

type Map = Vec<Vec<Tile>>;

fn make_map(map_width: i32, map_height: i32) -> Map {
    let mut map = vec![];
    
    // fill map with "unblocked" tiles
    for _ in 0..map_width {
        let column = vec![Tile{solid: false, opaque: false}; map_height as usize];
        map.push(column);
    }
    
    let w1 = (map_width*1/3) as usize;
    let w2 = (map_width*2/3) as usize;
    let h1 = (map_height/2) as usize;
    
    println!("placed first pillar at: {}", w1);

    // place two pillars to test the map
    map[w1][h1].solid = true;
    map[w1][h1].opaque = true;
    map[w2][h1].solid = true;
    map[w2][h1].opaque = true;

    map
}

fn render(  con: &mut RootConsole, 
            //center: &Vec2, 
            map: &Map, 
            creature: &Creature) {// later make list of characters
    // clear the screen
    con.clear();

    // draw map background
    for w in 0..map.len() {
        for h in 0..map[w].len() {
            if map[w][h].opaque==true {
                con.put_char(w as i32, h as i32, '#', BackgroundFlag::Set);
            }
        }
    }
    
    // draw creatures
    con.put_char(creature.pos.x, creature.pos.y, '@', BackgroundFlag::Set);

    // draw somed debug markers
    con.put_char(79, 49, 'Y', BackgroundFlag::Set);

    // show drawn to console
    con.flush();
}

fn main() {
    let mapsize = Vec2 {x: 80, y: 50};
    let mut con = RootConsole::initializer()
        .size(mapsize.x, mapsize.y)
        .title("libtcod Rust tutorial")
        .init(); // wtf kind of functional programming stuff is this?
    
    // create player
    let mut player = Creature {name: "Player".to_string(),
                                pos: Vec2 {x: 40, y: 25}};
    
    println!("player name is {}", player.name);
    
    // create map
    let map = make_map(mapsize.x, mapsize.y);

    while !con.window_closed() {
        render(&mut con, &map, &player);

        let keypress = con.wait_for_keypress(true);
        // libtcod 1.5.1 has a bug where `wait_for_keypress` emits two events:
        // one for key down and one for key up. So we ignore the "key up" ones.
        if keypress.pressed {
            match keypress {
                Key { code: Escape, .. } => break,
                Key { code: Up, .. } => player.pos.y -= 1,
                Key { code: Down, .. } => player.pos.y += 1,
                Key { code: Left, .. } => player.pos.x -= 1,
                Key { code: Right, .. } => player.pos.x += 1,
                _ => {}
            }
        }
    }
}
