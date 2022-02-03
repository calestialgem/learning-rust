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
            Grade::Aa => 400,
            Grade::Ba => 350,
            Grade::Bb => 300,
            Grade::Cb => 250,
            Grade::Cc => 200,
            Grade::Dc => 150,
            Grade::Dd => 100,
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
}

fn main() {
    println!("Hello, world!");
}
