use learnwell::{runner::Runner, agent::qlearning::QLearning, strategy::decliningrandom::DecliningRandom, environment::{Environment, EnvironmentDisplay}};
use show_image::{ImageView, ImageInfo};

/// Hike
/// based on https://adventofcode.com/2022/day/12, but modified so it is a bit easier (only ascents! thus will differ from original answer)
/// This is a harder learn than taxi (start there first)
/// Aim is to go from start point to end point, only by staying at same height (a-z) or ascending by maximum of 1 
/// The state set is much bigger, so it takes a lot of epochs to learn, and will take a few minutes to run
/// and sometimes it won't learn it. Try running it a few times, or increasing the epochs
/// Also, most likely, the optimum solution won't be reach, but it tries to find a better and 
/// better solution each time
/// 
/// This illustrates how to run_with_display.
/// We create an image for it to be displayed


fn main() {
    
    let epochs = 700_000;

    Runner::run_with_display(
        QLearning::new(0.2, 0.99,DecliningRandom::new(epochs, 0.005) ),
        Hike::new(),
        epochs,
        10
    );
    
    
}



const MAX: usize = 650;

pub struct Hike {
    grid: Vec<Vec<GridVal>>,
    end: Point,
    start: Point,
    best: Option<usize>,
    dis: f64,
    history: Vec<Point>,
    epoch: usize,
    best_route: Vec<Point>,
    pixels:Vec<u8>,

    state:MyState
}

#[derive(Clone, PartialEq, Eq)]
struct GridVal {
    height: u8,
    visited: bool,
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub enum MyAction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MyState {
    current: Point,
}

impl Default for MyState{
     fn default() -> MyState {
        MyState {
            current: Point { x: 0, y: 20 },
        }
    }
}


impl Default for Hike{
    fn default() -> Self {
        Self::new()
    }
}

impl Hike {
    pub fn new() -> Self {
        let grid= load_grid();
        Hike {
            grid,
            end: Point {
                x: 136,
                y: 20,
            },
            start: Point {
                x: 0,
                y: 20,
            },
            best: None,
            dis: 1000.,
            history: vec![],
            epoch: 0,
            best_route: vec![],
            pixels: vec![0u8;159*41*3],
            state:MyState::default()
        }
    }

   

    fn valid_actions(&self,  grid: &Vec<Vec<GridVal>>) -> Vec<MyAction> {
        let y = self.state.current.y as usize;
        let x = self.state.current.x as usize;

        //get current options
        let mut valid = vec![];
        //up
        if y > 0
            && !grid[y - 1][x].visited
            && (grid[y - 1][x].height == grid[y][x].height
                || grid[y - 1][x].height == grid[y][x].height + 1)
        {
            valid.push(MyAction::Up)
        }
        //down
        if y < grid.len() - 1
            && !grid[y + 1][x].visited
            && (grid[y + 1][x].height == grid[y][x].height
                || grid[y + 1][x].height == grid[y][x].height + 1)
        {
            valid.push(MyAction::Down)
        }
        //left
        if x > 0
            && !grid[y][x - 1].visited
            && (grid[y][x - 1].height == grid[y][x].height
                || grid[y][x - 1].height == grid[y][x].height + 1)
        {
            valid.push(MyAction::Left)
        }
        //right
        if x < grid[0].len() - 1
            && !grid[y][x + 1].visited
            && (grid[y][x + 1].height == grid[y][x].height
                || grid[y][x + 1].height == grid[y][x].height + 1)
        {
            valid.push(MyAction::Right)
        }

        valid
    }

    //for display
    fn update_pixels(&mut self){

        //height
        for (iy, y) in self.grid.iter().enumerate() {
            for (ix, p) in y.iter().enumerate() {
                let col = ((b'z' as f32 - p.height as f32) / 26.0 * 255.0) as u8;
                self.pixels[ix*3+iy*159*3]=col;//r
                self.pixels[ix*3+1+iy*159*3]=col;//g
                self.pixels[ix*3+2+iy*159*3]=col;//b
            }
        }
        //start
        self.pixels[self.start.x  as usize*3+self.start.y as usize*159*3]=0;
        self.pixels[self.start.x  as usize*3+1+self.start.y as usize*159*3]=255;
        self.pixels[self.start.x  as usize*3+2+self.start.y as usize*159*3]=0;
    
        //end
        self.pixels[self.end.x  as usize*3+self.end.y as usize*159*3]=255;
    

        for p in &self.best_route{
            let col = (150,150,250);
            self.pixels[p.x as usize*3+p.y as usize*159*3]=col.0;//r
            self.pixels[p.x as usize*3+1+p.y as usize*159*3]=col.1;//g
            self.pixels[p.x as usize*3+2+p.y as usize*159*3]=col.2;//b
        }

        for p in &self.history{
            let last = self.history.last().unwrap();
            
            let col = if last.x==self.end.x&&last.y==self.end.y{ (0,255,0)}else{(0,0,255)};
            self.pixels[p.x as usize*3+p.y as usize*159*3]=col.0;//r
            self.pixels[p.x as usize*3+1+p.y as usize*159*3]=col.1;//g
            self.pixels[p.x as usize*3+2+p.y as usize*159*3]=col.2;//b
        }
    }
    
}

fn load_grid() -> Vec<Vec<GridVal>>{
    let mut grid = vec![];
    for  line in GRID_RAW.split('\n') {
        let curr = line
            .as_bytes()
            .iter()
            .map(|&height| {
                GridVal { height, visited:false }
            })
            .collect();
        grid.push(curr);
    }
        grid
}

impl Environment<MyState, MyAction> for Hike {

    fn state(&self) -> MyState {
        self.state.clone()
    }

    fn actions(&self) -> Vec<MyAction> {
        vec![
            MyAction::Up,
            MyAction::Down,
            MyAction::Left,
            MyAction::Right,
        ]
        //may want to only return valid actions here, but found that instead, not moving and punishing in the reward step finds correct entry quicker
    }

    fn take_action_get_reward(&mut self, action: &MyAction) -> f64 {
        
        if self.valid_actions(&self.grid).contains(action) {
            
            match action {
                MyAction::Up => self.state.current.y -= 1,
                MyAction::Down => self.state.current.y += 1,
                MyAction::Left => self.state.current.x -= 1,
                MyAction::Right => self.state.current.x += 1,
            }
            let y = self.state.current.y as usize;
            let x = self.state.current.x as usize;
            self.grid[y][x].visited = true;

            let new_dis = dis(&self.state.current, &self.end);
            self.history.push(self.state.current.clone());

            //- for each step + if found
            let modi = if new_dis < 1. { 1000. } else { -100. };

             -new_dis + modi
        } else {
            -200. //punishment for not having valid step. This repeats for MAX number of steps
        }
    }

    fn should_stop(&mut self,  step: usize) -> bool {
        let state = &self.state;
        let finished= step > MAX || state.current.x == self.end.x && state.current.y == self.end.y ;
        
        if finished {
            self.update_pixels();
            on_finish(self,  step);
        }

        finished
    }

    fn reset(&mut self,  epoch: usize) {
        self.state=MyState::default();

        for y in self.grid.iter_mut() {
            for x in y {
                x.visited = false;
            }
        }
        self.grid[self.start.y as usize][self.start.x as usize].visited = true;

        self.history.clear();

        if epoch%50000==0{
        println!("Epoch {epoch}");
        }
    }
}

impl EnvironmentDisplay for Hike{
    fn get_image(&self) -> ImageView {
        let image = ImageView::new(ImageInfo::rgb8(159, 41), &self.pixels);
        image
    }
}

fn dis(a: &Point, b: &Point) -> f64 {
    ((a.x as f64 - b.x as f64).powf(2.) + (a.y as f64 - b.y as f64).powf(2.)).sqrt()
}

fn on_finish(env: &mut Hike,  step: usize) {
    
    
    let dis = dis(&env.state.current, &env.end);
    if dis < env.dis {
        println!("Got closer: {dis} {},{}",env.state.current.y,env.state.current.x);
         env.best_route = env.history.clone();
        env.dis = dis;
    }

    if dis < 1. {
        let best = if let Some(steps) = env.best {
            steps
        } else {
            usize::MAX
        };

        if step < best {
            env.best = Some(step);
            println!("Found in {step} steps finish:{},{}", env.state.current.y,env.state.current.x);
            env.best_route = env.history.clone();
        }
    }

    env.epoch += 1;
}


static GRID_RAW: &str = 
"abaaaaaaaaccccccccccccccccccaaaaaccccaaaaaaccccccccccccccccccccccaaaaaaaaaacccccccccccccccccccccccccccccccaaaaaccccccccccccccccccccccccccccccccccccccccccaaaaaa
abaaaaaaaacccccccccccccccccccaaaaaccccaaaacccccaaaacccccccccccccccaaaaaaaaaacccccccccccccccccccccccccccccaaaaaaccccccccccccccccccccccccccccccccccccccccccccaaaa
abccaaaaaaccccccccccccccccccaaaaaaccccaaaaccccaaaaaccccccccccaaaaaaaaaaaaaaacccccccccccccccccccccccccccccaaaacccccccccccccccccccccccccccccaaaccccccccccccccaaaa
abcaaaaaaaccccccccccccccccccaaaaccccccaccaccccaaaaaacccccccccaaaaaaaaaaaaaaacccccccccccccccccccccacccccccccaacccccccccccccccccccccccccccccaaaccccccccccccccaaaa
abccaacccaccccccccccccccccccccaaacccccccccccccaaaaaaccccccccccaaaaaaaaacaaacccccccccccccccccccaaaacccccccccccccccccccccccccaacccccccaaccccaaacccccccccccccaaaaa
abcaaaaaacccccccccccccccccccccccccccccccccccccaaaaaccccccccccaaaaaaaaaaccccaacaaccccccccccccccaaaaaacccccccccccccccccccccccaacccccccaaaacaaaaccccccccccccccaccc
abccaaaaacccccccccccccccccccccccccccccccccccaaccaaacccccccccaaaaaaaaaaaacccaaaaccccccccccccccccaaaaacccccccccccccccaacaaaaaaacccccccaaaaaaaaacccccccccccccccccc
abccaaaaaacccccccccccccccccccccccccccccaaacaaaccccccccccccccaaaaaaaaaaacccaaaaacccccccccccccccaaaaacccccccccccccaaaaaccaaaaaaaaccccccaaaaaalllllllcccaacccccccc
abccaaaaaaccccccaaaaacccccccccaaaccccccaaaaaaaccccccccccccccaaacaaacaaacccaaaaaaccccccccccccccaccaaccccccccccccccaaaaacaaaaaaaaajkkkkkkkkkklllllllccccaaaaacccc
abccaaaaacccccccaaaaacccccccccaaaaccccccaaaaaaccccccccaacaacccccaaacccccccacaaaaccccccccaaaccccccccccccccccccccccaaaaaccaaaaaaajjkkkkkkkkkkllssllllcccaaaaacccc
abcccaaaaccccccaaaaaacccccccccaaaaccccccaaaaaaaaccccccaaaaacccccaaccccccccccaacccccccccaaaacccccccccccccccaaccccaaaaaccaaaaaacjjjjkkkkkkkkssssssslllccaaaaccccc
abcccccccccccccaaaaaacccccccccaaaccccccaaaaaaaaacaaccccaaaaacccccccccccccccaaccccccccccaaaaccccccccccccccaaacccccccaaccaaaaaajjjjrrrrrrsssssssssslllcccaaaccccc
abcccccccccccccaaaaaacccccccccccccccccaaaaaaaaaaaaaaacaaaaaacccccccccccaaacaacccccccccccaaaccccaaacccccaaaaaaaaccccccccaacaaajjjrrrrrrrsssssuusssslmcccaaaacccc
abcccccccccccccccaacccccccccccccccaacaaaacaaaccaaaaaacaaaaccccccccccccccaaaaaccccccccccccccccccaaaaacccaaaaaaaaccccccccccccaajjjrrrruuursstuuuvsqqmmcddaaaacccc
abccccccccccccccccccccccccccccccccaaaaacccaaacccaaaaccccaaccccccccccccccaaaaaaacccccccccccccccaaaaaaccccaaaaaacccccccccccccccjjrrruuuuuuuuuuuuvvqqmmmdddccccccc
abcccccccccccccccccccccccacccccccccaaaaaccaaacccaaaaccccccccccccccccccccaaaaaaacccccccccccccccaaaaaaccccaaaaaacccccccccaaccccjjjrrtuuuuuuuuyyvvvqqmmmddddcccccc
abccccccccccccccccccccaaaaccccccccaaaaaacccccaacaccacccccccccccccccccccaaaaaaccccccccccccccccccaaaaaccccaaaaaaccccccccaaaccccjjjrrttuxxxuuxyyyvvqqmmmmdddcccccc
abcccccccccaacccccccccaaaaaaccccccaaaaccccccaaaccccccccccccccccccccccccaacaaaccccccccccccccccccaacaaccccaaccaaccccaaaaaaaccccjjjrrtttxxxxxyyyyvvqqqmmmddddccccc
abccccccccaaaacccccccccaaaacccccccccaaccccccaaacaaaccccccccccccccccccaaccccaacccccccccccccccccccccccccccccccccccccaaaaaaaaaacijjqrtttxxxxxyyyvvvqqqqmmmdddccccc
abcccccacaaaaaccccccccaaaaaccccccccccccccaaaaaaaaaacccccccccccccccccaaaccccccccccccccccccccccccccccccccccccccccccccaaaaaaaaaciiiqqqttxxxxxyyyvvvvqqqqmmmdddcccc
abcccccaaaaaaaaaacccccaacaaccccccccccccccaaaaaaaaaccccccccccccccaaacaaacccccccccccccccccccccccccccccccccccccccccccccaaaaaaaciiiqqqtttxxxzzzyyyyvvvqqqmmmdddcccc
abcccccaaaaaaaaaaccccccccccccaaccccccccccccaaaaaccccccccccccccccaaaaaaaaaacccccccaacccccccccccccaacccccccccccccccccaaaaaaccciiiqqqttxxxxyyyyyyyyvvvqqqmmmeddccc
abcccccccaaaaaacccccccccccaaaaccccccccccaaaaaaaaacccccccaaaacccccaaaaaaaaacccccaaaaccccccccccaacaaaccccccccccccccccaaaaaaaciiiqqqtttxxyyyyyyyyyvvvvqqqnnneeeccc
abcccccccaaaaaacccccccccccaaaaaaccccccccaaaaaaaaaaccccccaaaaccccccaaaaaaaccccccaaaaaaccccccccaaaaacccccccccccccccccaaccaaaciiiqqtttxxxxwwyyywwvvvvrrrnnnneeeccc
abcccccccaaaaaaccccccccccccaaaaacccccccaaaaaaacaaaccccccaaaacccccaaaaaacccccccccaaaaccccccccccaaaaaaccccaaccccccccccccccaaciiqqqtttxxxwwwyywwwwvvrrrrnnneeecccc
abccccccaaaaaaaaccccccccccaaaaaccccccccaaaaaaccccccccccccaaacccccaaaaaaacccccccaaaaaccccccccaaaaaaaaacccaaccccccccccccccccciiqqqtttttwwswwyywwrrrrrrnnnneeecccc
abccccccccccccacccccccccccaccaaccccaaccaaaaaacccccccccccaccccccccaaacaaacccccccaacaaccccccccaaaaacaaaaaaaacccccccccaacccccciiqqqqttssssswwwwwrrrrnnnnnneeeecccc
abcccccccccccccccccccccccccccccaaaaaaccccaacccccccaaacaaacccccccccccccaacaaacccccccccccccccccccaaaccaaaaaaaaccccaacaacccccciiiqqpppsssssswwwwrrrnnnnneeeeeccccc
abcccccccccccccccccccccccccccccaaaaaaaccccccccccccaaaaaaaccccccccccccccccaaacccccccccccccccccccaaaccaaaaaaaaacccaaaaacccccchhhhppppppppssswwwrroonnfeeeeacccccc
abccccccccccccccccccccaaaaaccccaaaaaaaaccccccccccccaaaaaaccccccccccccccaaaaaaaacccccccccccccccccccccaaaaaaaaaccccaaaaaaccccchhhhhpppppppsssssrroonfffeeaaaacccc
abccccccccccccccccccccaaaaacccccaaaaaaaccccccccccccaaaaaaaaccccccccccccaaaaaaaacccccccccccccccccccccaaaaaacccccaaaaaaaacccccchhhhhhhppppsssssrooofffffaaaaacccc
abcccccaacaaacccccccccaaaaaacccaaaaaacccccccccccccaaaaaaaaacccccccccccccaaaaacccccccccccccccccccccccaaaaaaaccccaaaaaccaccccccchhhhhhhhpppssssrooofffcaaaaaccccc
abcccccaaaaaacccccccccaaaaaacccaaaaaaccccccccccccaaaaaaaaaacccccccccccccaaaaaaccccccccccccccccccccccaccaaaccccccacaaaccaacccccccchhhhhgppooooooofffcaaaaacccccc
abcccccaaaaaacccccccccaaaaaaccccccaaacaacccccccccaaacaaaccccccccccaaacccaaaaaaccccccccccccccccccccccccccaaacccccccaaacaaaccccccccccchgggoooooooffffcaaaaaaccccc
abaccccaaaaaaaccccccccccaaccccccccaaaaaacccccccccccccaaaccccccccccaaaaccaaaccacaacaacccccccccccccccccccccccccccccccaaaaaaaaccccccccccggggoooooffffccaccaaaccccc
abacccaaaaaaaaccccccccccccccccccccaaaaaccccccccccccccaacccccccaaacaaaacccaaccccaaaaacccccccccccccccccccaacaacccccccaaaaaaaacccccccccccggggggggfffcccccccccccccc
abacccaaaaaaaaccccccccaaacccccccccaaaaaaccccccccccccccccccccccaaacaaaacaaaaccccaaaaaaccccccccaaccccccccaaaaaccccccccaaaaaaacccccccccccaaggggggffcccccccccccccca
abcccccccaaacccccccccaaaaaaccccccaaaaaaaacccccccccccccccccccaaaaaaaaaaaaaaaccccaaaaaaccccccacaaaacccccccaaaaacccccccaaaaaccccccccccccaaacgggggaccccccccccccccaa
abcccccccaaccccccccccaaaaaaccccccaaaaaaaacccccccaaacccccccccaaaaaaaaaaaaaaaacccaaaaaaccccccaaaaaaccccccaaaaaaccccccaaaaaaacccccccccccaaaccccaaaccccccccccaaacaa
abcccccccccccccccccccaaaaaccccccccccaaccccccccaaaaaccccccccccaaaaaaaaaaaaaaaaccccaaaccccccccaaaacccccccaaaaccccccccccccaaccccccccccccccccccccccccccccccccaaaaaa
abccccccccccccccccccccaaaaacccccccccaaccccccccaaaaaacccccccccaaaaaaaaaaaaaaaacccccccccccccccaaaacccccccccaacccccccccccccccccccccccccccccccccccccccccccccccaaaaa";
