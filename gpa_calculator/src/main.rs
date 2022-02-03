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

struct Course {
    credits: u32,
    points: u32,
}

impl Course {
    fn new(grade: Grade, credits: u32) -> Self {
        Course {
            credits,
            points: grade.points() * credits,
        }
    }
}

fn main() {
    println!("Hello, world!");
}
