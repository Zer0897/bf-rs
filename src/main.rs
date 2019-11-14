use std::io::prelude::*;

#[derive(PartialEq, Debug)]
enum Operation {
    MoveRight,
    MoveLeft,
    Increment,
    Decrement,
    Output,
    Input,
    JumpForward,
    JumpBack,
    NoOp,
}

impl Default for Operation {
    fn default() -> Self {
        Operation::NoOp
    }
}

impl From<char> for Operation {
    fn from(c: char) -> Self {
        match c {
            '>' => Operation::MoveRight,
            '<' => Operation::MoveLeft,
            '+' => Operation::Increment,
            '-' => Operation::Decrement,
            '.' => Operation::Output,
            ',' => Operation::Input,
            '[' => Operation::JumpForward,
            ']' => Operation::JumpBack,
            _ => Operation::NoOp,
        }
    }
}

fn parse<T: Read>(stream: T) -> impl Iterator<Item = Operation> {
    std::io::BufReader::new(stream)
        .bytes()
        .filter_map(|b| b.ok())
        .map(|b| char::from(b))
        .map(|c| Operation::from(c))
        .filter(|op| op != &Operation::NoOp)
}

struct Tape<T: Default> {
    cursor: usize,
    data: Vec<T>,
}

impl<T: Default> Tape<T> {
    fn new(data: Vec<T>) -> Self {
        Self { data, cursor: 0 }
    }

    fn mv_right(&mut self) {
        self.cursor += 1;
        if self.cursor >= self.data.len() {
            self.data.resize_with(self.data.len() * 2, Default::default);
        }
    }

    fn mv_left(&mut self) {
        self.cursor -= 1
    }

    fn cell(&self) -> &T {
        &self.data[self.cursor]
    }

    fn cell_mut(&mut self) -> &mut T {
        &mut self.data[self.cursor]
    }
}

struct Program {
    ops: Tape<Operation>,
    memory: Tape<u8>,
}

impl Program {
    fn new(program: Vec<Operation>) -> Self {
        // Allocate some memory to start with
        let mut memory = Vec::new();
        memory.resize(512, 0);

        Self {
            ops: Tape::new(program),
            memory: Tape::new(memory),
        }
    }

    fn inc(&mut self) {
        *self.memory.cell_mut() += 1
    }

    fn dec(&mut self) {
        *self.memory.cell_mut() -= 1
    }

    fn mvl(&mut self) {
        self.memory.mv_left()
    }

    fn mvr(&mut self) {
        self.memory.mv_right()
    }

    fn jpb(&mut self) {
        if *self.memory.cell() != 0 {
            while *self.ops.cell() != Operation::JumpForward {
                self.ops.mv_left();
            }
        }
    }

    fn jpf(&mut self) {
        if *self.memory.cell() == 0 {
            while *self.ops.cell() != Operation::JumpBack {
                self.ops.mv_right();
            }
        }
    }

    fn prt(&self) {
        print!("{}", char::from(*self.memory.cell()));
    }

    fn inp(&mut self) {
        let mut buff = String::new();
        std::io::stdin().read_line(&mut buff).unwrap();
        *self.memory.cell_mut() = buff.trim().parse().unwrap();
    }

    fn operate(&mut self) {
        match *self.ops.cell() {
            Operation::Increment => self.inc(),
            Operation::Decrement => self.dec(),
            Operation::MoveLeft => self.mvl(),
            Operation::MoveRight => self.mvr(),
            Operation::Output => self.prt(),
            Operation::Input => self.inp(),
            Operation::JumpForward => self.jpf(),
            Operation::JumpBack => self.jpb(),
            _ => {}
        }
    }

    fn step(&mut self) {
        self.operate();
        self.ops.mv_right();
    }

    fn run(&mut self) {
        while *self.ops.cell() != Operation::NoOp {
            self.step();
        }
    }
}

fn main() {
    let input = std::env::args().nth(1).unwrap();
    let reader = parse(std::fs::File::open(input).unwrap());
    let mut program = Program::new(reader.collect());
    program.run();
}
