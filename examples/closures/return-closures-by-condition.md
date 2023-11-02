# Example: return different closures by some condition
```Rust
fn gen_nums(low: u32, upper: u32) -> Box<dyn Iterator<Item = u32>> {
    if upper <= 100 {
        Box::new((low..upper).filter(|x| x % 10 == 0))
    } else {
        Box::new((low..upper).filter(|x| x % 100 == 0))
    }
}

fn main() {
    for n in gen_nums(100, 1001) {
        println!("{}", n);
    }
    for n in gen_nums(55, 99) {
        println!("{}", n);
    }
}
```