use ferris_says::say; // from the previous step
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let out = b"Hello fellow Rustaceans!";
    let width = 24;

    let mut writer = BufWriter::new(stdout.lock());
    say(out, width, &mut writer).unwrap();

    let mut sum = 0;
    for number in (1..4){
        println!("{}", number);
        sum += number;
    }
    println!("Sum is {}", sum);
}