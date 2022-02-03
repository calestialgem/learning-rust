enum Grade {
    Aa,
    Ba,
    Bb,
    Cb,
    Cc,
    Dc,
    Dd,
    F,
}

impl Grade {
    fn points(&self) -> u32 {
        match *self {
            Grade::Aa => 40,
            Grade::Ba => 35,
            Grade::Bb => 30,
            Grade::Cb => 25,
            Grade::Cc => 20,
            Grade::Dc => 15,
            Grade::Dd => 10,
            Grade::F => 0,
        }
    }
}

fn main() {
    println!("Hello, world!");
}
