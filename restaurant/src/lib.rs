// in create
fn serve_order() {}

// in crate
mod back_of_house {
    // in crate::back_of_house
    // thus super means crate
    fn fix_incorrect_order() {
        cook_order();
        // access crate::serve_order by goin up by one
        super::serve_order();
    }

    fn cook_order() {}
}
