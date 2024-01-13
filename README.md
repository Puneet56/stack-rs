# Stack-rs

Simple stack based virtal machine in rust.

## Supported Instructions

``` rust
enum Instruction {
    Push(isize),
    Add,
    Print,
    Sub,
    Mul,
    Div,
    Mod,
}
```
## Sample Program
```rust    
let program = vec![
    Instruction::Push(34),
    Instruction::Push(35),
    Instruction::Add,
    Instruction::Print,
];
```
Output:
```bash
$ cargo run
   Compiling stack-rs v0.1.0 (/Users/puneet/projects/stack-rs)
    Finished dev [unoptimized + debuginfo] target(s) in 0.23s
     Running `target/debug/stack-rs`
69
```
