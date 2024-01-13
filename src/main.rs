enum Instruction {
    Push(isize),
    Add,
    Print,
    Sub,
    Mul,
    Div,
    Mod,
}

fn run_program(instructions: Vec<Instruction>) -> Result<(), String> {
    let mut stack: Vec<isize> = vec![];

    for instruction in instructions {
        match instruction {
            Instruction::Push(s) => stack.push(s),
            Instruction::Add => {
                if stack.len() < 2 {
                    return Err(
                        "Not enough elements in stack to execute Add instruction".to_string()
                    );
                }
                let left = stack.pop().unwrap();
                let right = stack.pop().unwrap();
                stack.push(left + right)
            }
            Instruction::Print => {
                if stack.len() < 1 {
                    return Err("Stack is empty. Nothing to print".to_string());
                }

                println!("{}", stack.pop().unwrap())
            }

            Instruction::Sub => {
                if stack.len() < 2 {
                    return Err(
                        "Not enough elements in stack to execute Sub instruction".to_string()
                    );
                }
                let left = stack.pop().unwrap();
                let right = stack.pop().unwrap();
                stack.push(left - right)
            }

            Instruction::Mul => {
                if stack.len() < 2 {
                    return Err(
                        "Not enough elements in stack to execute Mul instruction".to_string()
                    );
                }
                let left = stack.pop().unwrap();
                let right = stack.pop().unwrap();
                stack.push(left * right)
            }

            Instruction::Div => {
                if stack.len() < 2 {
                    return Err(
                        "Not enough elements in stack to execute Div instruction".to_string()
                    );
                }
                let left = stack.pop().unwrap();
                let right = stack.pop().unwrap();
                stack.push(left / right)
            }

            Instruction::Mod => {
                if stack.len() < 2 {
                    return Err(
                        "Not enough elements in stack to execute Mod instruction".to_string()
                    );
                }
                let left = stack.pop().unwrap();
                let right = stack.pop().unwrap();
                stack.push(left % right)
            }
        }
    }

    Ok(())
}

fn main() {
    let program = vec![
        Instruction::Push(34),
        Instruction::Push(35),
        Instruction::Add,
        Instruction::Print,
        Instruction::Push(50),
        Instruction::Push(60),
        Instruction::Sub,
        Instruction::Print,
        Instruction::Push(4),
        Instruction::Push(2),
        Instruction::Mul,
        Instruction::Print,
        Instruction::Push(3),
        Instruction::Push(10),
        Instruction::Div,
        Instruction::Print,
        Instruction::Push(10),
        Instruction::Push(3),
        Instruction::Mod,
        Instruction::Print,
    ];

    if let Err(e) = run_program(program) {
        println!("Error: {}", e);
    }
}
