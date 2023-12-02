mod one;
mod tests;
mod two;

fn main() {
    println!("starting");
    let input = &"input1.txt"[..];
    let _ = two::runtwo(input);
}
