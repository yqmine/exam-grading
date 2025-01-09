fn main() {
    let mut res:i32 = 42;
    let option = Some(12);
    // TODO: Fix the Clippy lint.
    if let Some(x) = option {
        res += x;
    }

    println!("{res}");
}
