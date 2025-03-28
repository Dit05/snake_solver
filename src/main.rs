
const LENGTHS: &[u32] = &[2, 1, 1, 2, 1, 2, 1, 1, 2, 2, 1, 1, 1, 2, 2, 2, 2];

#[derive(Clone, Copy)]
#[derive(PartialEq)]
#[derive(Debug)]
enum Dir {
    Right,
    Left,
    Up,
    Down,
    Forward,
    Backward
}

impl Dir {

    pub fn get_possible_turns(&self) -> [Dir; 4] {
        let mut turns = [Dir::Right; 4];
        let mut i = 0;

        for dir in [Dir::Right, Dir::Left, Dir::Up, Dir::Down, Dir::Forward, Dir::Backward] {
            if dir != *self && dir != self.invert() {
                turns[i] = dir;
                i += 1;
            }
        }

        turns
    }

    pub fn first() -> Dir {
        Dir::Right
    }

    pub fn invert(&self) -> Dir {
        match self {
            Dir::Right => Dir::Left,
            Dir::Left => Dir::Right,
            Dir::Up => Dir::Down,
            Dir::Down => Dir::Up,
            Dir::Forward => Dir::Backward,
            Dir::Backward => Dir::Forward,
        }
    }

    pub fn get_next(&self) -> Option<Dir> {
        match self {
            Dir::Right => Some(Dir::Left),
            Dir::Left => Some(Dir::Up),
            Dir::Up => Some(Dir::Down),
            Dir::Down => Some(Dir::Forward),
            Dir::Forward => Some(Dir::Backward),
            Dir::Backward => None,
        }
    }

    pub fn to_vec3(&self) -> Vec3 {
        match self {
            Dir::Right => Vec3 { x: 1, y: 0, z: 0},
            Dir::Left => Vec3 { x: -1, y: 0, z: 0 },
            Dir::Up => Vec3 { x: 0, y: 1, z: 0 },
            Dir::Down => Vec3 { x: 0, y: -1, z: 0 },
            Dir::Forward => Vec3 { x: 0, y: 0, z: -1 },
            Dir::Backward => Vec3 { x: 0, y: 0, z: 1 },
        }
    }

}


#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone, Copy)]
struct Vec3 {
    pub x: i32,
    pub y: i32,
    pub z: i32
}

impl std::ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl std::ops::Mul<i32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: i32) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs
        }
    }
}


struct Cube {
    cells: [bool; (Cube::SIZE * Cube::SIZE * Cube::SIZE) as usize]
}

impl Cube {

    const SIZE: u32 = 4;


    pub fn new() -> Cube {
        Cube {
            cells: [false; (Self::SIZE * Self::SIZE * Self::SIZE) as usize]
        }
    }


    pub fn get_mut(&mut self, vec: &Vec3) -> &mut bool {
        let index = (vec.z * (Cube::SIZE * Cube::SIZE) as i32) + (vec.y * Cube::SIZE as i32) + vec.x;
        &mut self.cells[TryInto::<usize>::try_into(index).unwrap()]
    }

    pub fn get(&self, vec: &Vec3) -> &bool {
        let index = (vec.z * (Cube::SIZE * Cube::SIZE) as i32) + (vec.y * Cube::SIZE as i32) + vec.x;
        &self.cells[TryInto::<usize>::try_into(index).unwrap()]
    }

    pub fn is_inside(&self, vec: &Vec3) -> bool {
        vec.x >= 0 && vec.x < Cube::SIZE as i32 &&
        vec.y >= 0 && vec.y < Cube::SIZE as i32 &&
        vec.z >= 0 && vec.z < Cube::SIZE as i32
    }


    pub fn try_move_once(&mut self, dir: &Dir, pos: &mut Vec3) -> bool {
        let next_pos = *pos + dir.to_vec3();
        assert_ne!(*pos, next_pos);

        if self.is_inside(&next_pos) && *self.get(&next_pos) == false {
            *self.get_mut(pos) = true;
            *pos = next_pos;
            true
        } else {
            false
        }
    }

}


fn advance_attempt(mut moves: Vec<Dir>) -> Option<Vec<Dir>> {
    while let Some(last) = moves.pop() {
        if let Some(next_last) = last.get_next() {
            moves.push(next_last);
            return Some(moves);
        }
    }
    None
}

fn are_moves_valid(moves: &Vec<Dir>, lengths: &[u32]) -> bool {
    let mut cube = Cube::new();
    let mut pos = Vec3 { x: 0, y: 0, z: 0 };
    *cube.get_mut(&pos) = true;

    for i in 0..moves.len() {
        for _ in 0..lengths[i] {
            if !cube.try_move_once(&moves[i], &mut pos) {
                return false;
            }
        }
    }

    true
}

fn solve(lengths: &[u32]) -> Option<Vec<Dir>> {
    let mut moves = Vec::<Dir>::new();
    let mut length_record = 0;

    loop {
        if are_moves_valid(&moves, lengths) {
            if moves.len() > length_record {
                length_record = moves.len();
            }

            if moves.len() >= lengths.len() {
                return Some(moves);
            }

            moves.push(Dir::first());
        } else {
            if let Some(next_moves) = advance_attempt(moves) {
                moves = next_moves;
            } else {
                return None; // Genuis! `moves` is always reinitialized.
            }
        }
    }
}

fn solve_default() {
    solve(LENGTHS);
}

fn look_for_solvables(start: usize, stride: usize) {
    let mut lengths = Vec::<u32>::new();

    fn next_lengths(mut lengths: Vec<u32>) -> Vec<u32> {
        let mut i = 0;
        while i < lengths.len() {
            match lengths[i] {
                1 => {
                    lengths[i] = 2;
                    return lengths;
                },
                2 => {
                    lengths[i] = 3;
                    return lengths;
                },
                3 => {
                    lengths[i] = 1;
                    i += 1;
                },
                _ => unreachable!()
            }
        }
        lengths.push(1);
        lengths
    }

    for _ in 0..start {
        lengths = next_lengths(lengths);
    }

    const EXPECTED_SUM: u32 = Cube::SIZE * Cube::SIZE * Cube::SIZE;

    fn sum_lengths(lengths: &[u32]) -> u32 {
        lengths.iter().cloned().fold(1, |acc, x| acc + x)
    }
    //assert_eq!(sum_lengths(LENGTHS), EXPECTED_SUM);

    let mut i: usize = 0;
    loop {
        i += 1;
        if i % 10_000_000 == 0 {
            /*const TOTAL: usize = 1 << 17;
            let percent = i as f64 / TOTAL as f64;
            println!("{:.2}%", percent * 100.0);*/
            eprintln!("({}) i = {}, len = {}", start, i, lengths.len());
        }

        if sum_lengths(&lengths) == EXPECTED_SUM {
            if let Some(_solution) = solve(&lengths) {
                //println!("Solvable lengths: {:?}, solution: {:?}", lengths, solution);
                print!("|");
                for n in &lengths {
                    match n {
                        1 => print!(" "),
                        2 => print!("."),
                        3 => print!(":"),
                        _ => unreachable!()
                    }
                }
                println!("|");
            }
        }

        /*if let Some(next) = next_lengths(lengths) {
            lengths = next;
        } else {
            break;
        }*/
        for _ in 0..stride {
            lengths = next_lengths(lengths);
        }
    }
    //println!("Fin.");
}


fn main() {
    let n_threads: usize = match std::thread::available_parallelism() {
        Ok(n) => {
            eprintln!("parallelism: {}", n);
            n.into()
        },
        Err(e) => {
            const ASSUMED: usize = 4;
            eprintln!("couldn't figure out parallelism (reason: {}), assuming {}", e, ASSUMED);
            ASSUMED
        }
    };

    let mut handles = Vec::<std::thread::JoinHandle<()>>::new();

    for i in 0..n_threads {
        let a = i.clone();
        let b = n_threads.clone();
        let handle = std::thread::spawn(move || look_for_solvables(a, b));
        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.join();
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_a_few_times() {
        let mut cube = Cube::new();
        let mut pos = Vec3 { x: 0, y: 0, z: 0 };
        assert!(cube.try_move_once(&Dir::Backward, &mut pos));
        assert!(cube.try_move_once(&Dir::Backward, &mut pos));
        assert!(cube.try_move_once(&Dir::Up, &mut pos));
        assert!(cube.try_move_once(&Dir::Up, &mut pos));
        assert!(cube.try_move_once(&Dir::Right, &mut pos));
        assert!(cube.try_move_once(&Dir::Right, &mut pos));
    }

    #[test]
    fn obviously_valid_moves() {
        assert!(are_moves_valid(&vec![], &LENGTHS));
        assert!(are_moves_valid(&vec![Dir::Backward], &LENGTHS));
        assert!(are_moves_valid(&vec![Dir::Backward, Dir::Up], &LENGTHS));
        assert!(are_moves_valid(&vec![Dir::Backward, Dir::Up, Dir::Right], &LENGTHS));
    }
}
