// functions3.rs
// Execute `rustlings hint functions3` or use the `hint` watch subcommand for a hint.

fn main() {
    call_me(8);
}

fn call_me(num: u32) {
    (0..num).for_each(|i| {
        println!("Ring! Call number {}", i + 1);
    });
}
