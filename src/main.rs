use std::io;

fn main() {
    let mut table = vec![1, 0, 1];
    let mut a = 0;
    let mut b = 1;
    for _ in 0..39 {
        (a, b) = (b, a + b);
        table.push(b);
    }

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let test_case_count = line.trim().parse::<usize>().unwrap();
    for _ in 0..test_case_count {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let index = line.trim().parse::<usize>().unwrap();
        println!("{} {}", table[index], table[index + 1]);
    }
}
