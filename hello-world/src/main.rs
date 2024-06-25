const LAMPORTS_PER_SOL: u64 = 1_000_000_000;
 
fn main() {
    println!("LAMPORTS_PER_SOL: {}", LAMPORTS_PER_SOL);   
    println!("Hello, world!");



    let x: u32;

    let y: &str;

    let mut x:u32 = 20;
    println!("x: {x}");
    x = 20;

    let size = if x < 20 {"small"} else {"big"};
    println!("size: {size}");
}
