mod front_of_house {
    pub mod hosting {
        pub struct Visitors(pub u32);

        pub fn add_to_waitlist(group: Visitors) {}
    }
}

use self::front_of_house::hosting;
use self::front_of_house::hosting::Visitors as Group;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist(Group(3));
    hosting::add_to_waitlist(Group(2));
    hosting::add_to_waitlist(Group(6));
}
