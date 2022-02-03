struct Points {
    whole: u32,
    fraction: u32,
}

impl Points {
    const SCALE: u32 = 1000;
    const HALF: u32 = Self::SCALE / 2;
    const ZERO: Points = Points::new(0, 0);

    const fn new(whole: u32, fraction: u32) -> Self {
        Self {
            whole: whole + fraction / Self::SCALE,
            fraction: fraction % Self::SCALE,
        }
    }

    fn as_fraction(&self) -> u32 {
        self.whole * Self::SCALE + self.fraction
    }

    fn add(&self, other: &Self) -> Self {
        Self::new(self.whole + other.whole, self.fraction + other.fraction)
    }

    fn mul(&self, scalar: u32) -> Self {
        Self::new(0, self.as_fraction() * scalar)
    }

    fn div(&self, scalar: u32) -> Self {
        Self::new(0, self.as_fraction() / scalar)
    }

    fn round(&self) -> Self {
        let round_up = self.fraction % 10 >= 5;
        let tens = self.fraction / 10 + round_up as u32;
        Self::new(self.whole, tens * 10)
    }

    fn print(&self) {
        if self.fraction == 0 {
            print!("{}", self.whole);
        } else {
            let mut scale = Self::SCALE;
            loop {
                scale /= 10;
                if scale < 10 {
                    print!("{}.{}", self.whole, self.fraction);
                    break;
                }
                if self.fraction % scale == 0 {
                    print!("{}.{}", self.whole, self.fraction / scale);
                    break;
                }
            }
        }
    }
}

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
    fn points(&self) -> Points {
        match *self {
            Grade::Aa => Points::new(4, 0),
            Grade::Ba => Points::new(3, Points::HALF),
            Grade::Bb => Points::new(3, 0),
            Grade::Cb => Points::new(2, Points::HALF),
            Grade::Cc => Points::new(2, 0),
            Grade::Dc => Points::new(1, Points::HALF),
            Grade::Dd => Points::new(1, 0),
            Grade::F => Points::ZERO,
        }
    }
}

struct Course {
    credits: u32,
    points: Points,
}

impl Course {
    fn new(grade: Grade, credits: u32) -> Self {
        Course {
            credits,
            points: grade.points().mul(credits),
        }
    }
}

struct Semester {
    name: String,
    credits: u32,
    points: Points,
    average: Points,
}

impl Semester {
    fn new(name: String, courses: &[Course]) -> Self {
        let mut credits = 0;
        let mut points = Points::ZERO;
        for course in courses {
            credits += course.credits;
            points = points.add(&course.points);
        }
        let average = points.div(credits);
        Semester {
            name,
            credits,
            points,
            average,
        }
    }

    fn print(&self) {
        print!("[{}] Credits: {}, Points: ", self.name, self.credits);
        self.points.round().print();
        print!(", SPA: ");
        self.average.round().print();
        println!();
    }
}

struct Overall {
    credits: u32,
    points: Points,
    average: Points,
}

impl Overall {
    fn new(semesters: &[Semester]) -> Self {
        let mut credits = 0;
        let mut points = Points::ZERO;
        for semester in semesters {
            credits += semester.credits;
            points = points.add(&semester.points);
        }
        let average = points.div(credits);
        Overall {
            credits,
            points,
            average,
        }
    }

    fn print(&self) {
        print!("[Overall] Credits: {}, Points: ", self.credits);
        self.points.round().print();
        print!(", SPA: ");
        self.average.round().print();
        println!();
    }
}

fn main() {
    use Grade::*;
    let fall_18 = Semester::new(
        String::from("2018/2019-1"),
        &[
            Course::new(Ba, 4),
            Course::new(Aa, 3),
            Course::new(Aa, 3),
            Course::new(Aa, 4),
            Course::new(Ba, 4),
        ],
    );
    fall_18.print();
}
