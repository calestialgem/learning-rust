struct Points {
    whole: u32,
    fraction: u32,
}

impl Points {
    const scale: u32 = 1000;
    const half: u32 = Self::scale / 2;

    fn new(whole: u32, fraction: u32) -> Self {
        Self {
            whole: whole + fraction / Self::scale,
            fraction: fraction % Self::scale,
        }
    }

    fn as_fraction(&self) -> u32 {
        self.whole * Self::scale + self.fraction
    }

    fn add(&self, other: &Self) -> Self {
        Self::new(self.whole + other.whole, self.fraction + other.fraction)
    }

    fn mul(&self, other: &Self) -> Self {
        Self::new(0, self.as_fraction() * other.as_fraction() / Self::scale)
    }

    fn div(&self, other: u32) -> Self {
        Self::new(0, self.as_fraction() / other)
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
    fn points(&self) -> u32 {
        match *self {
            Grade::Aa => 4000,
            Grade::Ba => 3500,
            Grade::Bb => 3000,
            Grade::Cb => 2500,
            Grade::Cc => 2000,
            Grade::Dc => 1500,
            Grade::Dd => 1000,
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

struct Semester {
    name: String,
    credits: u32,
    points: u32,
    average: u32,
}

impl Semester {
    fn new(name: String, courses: &[Course]) -> Self {
        let mut credits = 0;
        let mut points = 0;
        for course in courses {
            credits += course.credits;
            points += course.points;
        }
        Semester {
            name,
            credits,
            points,
            average: points / credits,
        }
    }

    fn print(&self) {
        println!(
            "[{}] Credits: {}, Points: {}.{}, SPA: {}",
            self.name, self.credits, self.points, self.average
        );
    }
}

struct Overall {
    credits: u32,
    points: u32,
    average: u32,
}

impl Overall {
    fn new(semesters: &[Semester]) -> Self {
        let mut credits = 0;
        let mut points = 0;
        for semester in semesters {
            credits += semester.credits;
            points += semester.points;
        }
        Overall {
            credits,
            points,
            average: points / credits,
        }
    }

    fn print(&self) {
        println!(
            "[Overall] Credits: {}, Points: {}, GPA: {}",
            self.credits, self.points, self.average
        );
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
