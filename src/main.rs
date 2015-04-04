use std::io::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;

type P<T> = Rc<RefCell<T>>;
fn P<T>(val: T) -> P<T> {
    return Rc::new(RefCell::new(val));
}

#[derive(Debug, Clone)]
enum Cell {
    Rock,
    Empty,
    Gold(u8),
    Player(P<Player>),
    Emu(P<Emu>),
    Wall,
}

impl Cell {
    fn is_player(&self) -> bool {
        if let &Cell::Player(_) = self {
            true
        } else {
            false
        }
    }
}

#[derive(Debug)]
struct Game {
    map: Map,
    player: Rc<RefCell<Player>>,
    emus: Vec<Rc<RefCell<Emu>>>,
}

enum Dir {
    Left,
    Right,
    Up,
    Down,
}

impl Game {
    fn move_player(&mut self, dir: Dir) {
        let (ox, oy) = self.player.borrow().pos;
        let (x, y) = match dir {
            Dir::Left => (ox + 1, oy),
            Dir::Right => (ox - 1, oy),
            Dir::Up => (ox, oy + 1),
            Dir::Down => (ox, oy - 1),
        };

        if x > self.map.width {
            return;
        }
        if y > self.map.height {
            return;
        }

        self.player.borrow_mut().pos = (x, y);

        let new = self.map.data[oy * self.map.width + ox]
            .iter()
            .cloned()
            .inspect(|cell| println!("{:?}", cell))
            .filter(|cell| !cell.is_player())
            .collect();
        std::mem::replace(&mut self.map.data[oy * self.map.width + ox],
                          new);
        self.map.data[y * self.map.width + x].push(Cell::Player(self.player.clone()));
    }
}

#[derive(Debug, Clone)]
struct Player {
    pos: (usize, usize),
    hp: usize,
    dam: usize,
    gold: usize,
}

#[derive(Debug, Clone)]
struct Emu {
    hp: usize,
    dam: usize,
}

#[derive(Debug)]
struct Map {
    width: usize,
    height: usize,
    data: Vec<Vec<Cell>>
}

fn from_loadfile(s: &[u8], width: usize, height: usize) -> Game {
    let mut m = Map { width: width, height: height, data: Vec::new() };
    let mut player = None;
    let mut emus = Vec::new();

    for h in 0..height {
        for w in 0..width {
            match s[(h * width + w) as usize] {
                b' ' => m.data.push(vec![Cell::Rock]),
                b'.' => m.data.push(vec![Cell::Empty]),
                b'@' => {
                    let pl = P(Player {
                                pos: (w, h),
                                hp: 42,
                                dam: 7,
                                gold: 0
                            });
                    m.data.push(vec![Cell::Player(pl.clone())]);
                    m.data.last_mut().unwrap().push(Cell::Empty);
                    if player.is_none() {
                        player = Some(pl);
                    } else {
                        panic!("More than one player?");
                    }
                },
                b'E' => {
                    let em = P(Emu { hp: 42, dam: 1 });
                    m.data.push(vec![Cell::Emu(em.clone())]);
                    emus.push(em);
                }
                b'*' => m.data.push(vec![Cell::Gold(42)]),
                b'#' => m.data.push(vec![Cell::Wall]),
                b'\n' => { },
                _ => panic!("Crap loadfile, make another")
            }
        }
    }

    return Game { map: m, player: player.unwrap(), emus: emus };
}

impl Map {
    fn draw(&self) {
        for h in 0..self.height {
            for w in 0..self.width {
                let cell_stuff = &self.data[h * self.width + w];
if cell_stuff.iter().any(
    |cell| if let &Cell::Player(_) = cell { true } else { false }) {
                    print!("@");
                } else {
                    match cell_stuff[0] {
                        Cell::Rock => print!(" "),
                        Cell::Empty => print!("."),
                        Cell::Gold(_) => print!("*"),
                        Cell::Player(_) => unreachable!(),
                        Cell::Emu(_) => print!("E"),
                        Cell::Wall => print!("#"),
                    }
                }
            }
            println!("");
        }
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let filename = &args[1];
    let width = args[2].parse().unwrap(); //.expect("Bad width");
    let height = args[3].parse().unwrap(); //.expect("Bad height");

    let mut f = std::fs::File::open(&filename).unwrap();
    let mut loaded = Vec::new();
    f.read_to_end(&mut loaded);

    let mut game = from_loadfile(&loaded, width, height);
    let mut stdin = std::io::stdin();
    let mut line = String::new();

    loop {
        game.map.draw();
        println!("Input: ");
        stdin.read_line(&mut line);
        if line.len() == 0 {
            return;
        }

        match line.as_bytes()[0] {
            b'h' => { game.move_player(Dir::Left) },
            b'j' => { game.move_player(Dir::Up) },
            b'k' => { game.move_player(Dir::Down) },
            b'l' => { game.move_player(Dir::Right) },
            b',' => {  },
            b'q' => { println!("I fall on my sword."); return; }
            c => { println!("I don't know how to {}", c as char); }
        }

        line.truncate(0);
    }
}
