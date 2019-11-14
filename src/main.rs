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

impl From<u8> for Operation {
    fn from(n: u8) -> Self {
        Self::from(char::from(n))
    }
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
            self.data.resize_with(self.data.len() * 2, T::default);
        }
    }

    fn mv_left(&mut self) {
        self.cursor -= 1;
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

    /// bf increment `+`
    fn inc(&mut self) {
        *self.memory.cell_mut() = self.memory.cell().wrapping_add(1)
    }

    /// bf decrement `-`
    fn dec(&mut self) {
        *self.memory.cell_mut() = self.memory.cell().wrapping_sub(1)
    }

    /// bf move left `<`
    fn mvl(&mut self) {
        self.memory.mv_left()
    }

    /// bf move right `>`
    fn mvr(&mut self) {
        self.memory.mv_right()
    }

    /// bf jump backward `]`
    fn jpb(&mut self) {
        if *self.memory.cell() != 0 {
            let mut count = 1;
            while count > 0 {
                self.ops.mv_left();

                if *self.ops.cell() == Operation::JumpBack {
                    count += 1;
                } else if *self.ops.cell() == Operation::JumpForward {
                    count -= 1;
                }
            }
        }
    }

    /// bf jump foward `[`
    fn jpf(&mut self) {
        if *self.memory.cell() == 0 {
            let mut count = 1;
            while count > 0 {
                self.ops.mv_right();

                if *self.ops.cell() == Operation::JumpForward {
                    count += 1;
                } else if *self.ops.cell() == Operation::JumpBack {
                    count -= 1;
                }
            }
        }
    }

    /// bf output `.`
    fn prt(&self) {
        print!("{}", char::from(*self.memory.cell()));
    }

    /// bf input `,`
    fn inp(&mut self) {
        let mut buff = String::new();
        std::io::stdin().read_line(&mut buff).unwrap();
        *self.memory.cell_mut() = buff.trim().parse().unwrap();
    }

    /// Execute the current operation. Should not be used directly,
    /// use `step` instead.
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

    /// Execute the next operation
    fn step(&mut self) {
        self.operate();
        self.ops.mv_right();
    }

    /// Execute all operations
    fn run(&mut self) {
        while *self.ops.cell() != Operation::NoOp {
            self.step();
        }
    }
}

fn parse<T: Read>(stream: T) -> impl Iterator<Item = Operation> {
    std::io::BufReader::new(stream)
        .bytes()
        // Get valid bytes
        .filter_map(|b| b.ok())
        // Convert to operations
        .map(|c| Operation::from(c))
        // Ignore NoOps
        .filter(|op| op != &Operation::NoOp)
}

fn main() {
    let input = std::env::args().nth(1).expect("Expected a file.");
    let ops = parse(std::fs::File::open(input).expect("Invalid file path."));
    let mut program = Program::new(ops.collect());
    program.run();
}
