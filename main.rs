#[derive(Debug, Clone, Copy, PartialEq)]
enum Op { Add, Sub, Mul, Div }

impl Op {
    fn from_char(c: char) -> Option<Op> {
        match c {
            '+' => Some(Op::Add),
            '-' => Some(Op::Sub),
            '*' => Some(Op::Mul),
            '/' => Some(Op::Div),
            _ => None,
        }
    }

    fn symbol(self) -> char {
        match self {
            Op::Add => '+',
            Op::Sub => '-',
            Op::Mul => '*',
            Op::Div => '/',
        }
    }

    fn precedence(self) -> u8 {
        match self {
            Op::Add | Op::Sub => 1,
            Op::Mul | Op::Div => 2,
        }
    }

    fn apply(self, a: f64, b: f64) -> Option<f64> {
        match self {
            Op::Add => Some(a + b),
            Op::Sub => Some(a - b),
            Op::Mul => Some(a * b),
            Op::Div => {
                if b == 0.0 {
                    None
                } else {
                    Some(a / b)
                }
            }
        }
    }
}

fn evaluate_simple(a: f64, op: char, b: f64) -> Option<f64> {
    Op::from_char(op)?.apply(a, b)
}

#[derive(Debug)]
struct Stack { 
    items: Vec<f64> 
}

impl Stack {
    fn new() -> Self {
        Self { items: Vec::new() }
    }

    fn push(&mut self, v: f64) {
        self.items.push(v);
    }

    fn pop(&mut self) -> Option<f64> {
        self.items.pop()
    }

    fn peek(&self) -> Option<&f64> {
        self.items.last()
    }

    fn len(&self) -> usize {
        self.items.len()
    }

    fn apply(&mut self, op: Op) -> Option<()> {
        if self.items.len() < 2 {
            return None;
        }
        
        let b = self.items[self.items.len() - 1];
        let a = self.items[self.items.len() - 2];

        let result = op.apply(a, b)?;

        self.pop();
        self.pop();
        self.push(result);
        
        Some(())
    }
}

fn task10_computation() {
    let mut stack = Stack::new();
    
    stack.push(2.0);
    stack.push(3.0);
    stack.apply(Op::Add);
    
    stack.push(4.0);
    stack.apply(Op::Mul);
    
    stack.push(1.0);
    stack.apply(Op::Sub);
    
    println!("Final stack result: {:?}", stack.peek());

    println!("Emptying the stack...");
    while let Some(v) = stack.pop() {
        println!("Popped: {}", v);
    }
}

fn main() {
    println!("--- Part 1 Tests ---");
    println!("from_char('*'): {:?}", Op::from_char('*'));
    println!("from_char('x'): {:?}", Op::from_char('x'));
    println!("Op::Div symbol: {}", Op::Div.symbol());
    println!("Op::Sub precedence: {}", Op::Sub.precedence());
    println!("Op::Sub apply (5.0, 3.0): {:?}", Op::Sub.apply(5.0, 3.0));
    println!("Op::Div apply (1.0, 0.0): {:?}", Op::Div.apply(1.0, 0.0));
    println!("evaluate_simple (6, '/', 4): {:?}", evaluate_simple(6.0, '/', 4.0));
    println!("evaluate_simple (1, '/', 0): {:?}", evaluate_simple(1.0, '/', 0.0));
    println!("evaluate_simple (1, '?', 2): {:?}", evaluate_simple(1.0, '?', 2.0));

    println!("\n--- Part 2 & Task 10 Tests ---");
    task10_computation();
}