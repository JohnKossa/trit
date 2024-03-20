mod trit;
mod numbers;

struct AdderResult {
    sum: trit::Trit,
    carry: trit::Trit
}

fn half_adder(a: trit::Trit, b: trit::Trit) -> AdderResult{
    return AdderResult{
        sum: a.xor(&b),
        carry: a.and(&b)
    };
}

fn main() {
    println!("Hello, world!");
}
