extern crate libhandy;
extern crate gtk;
use libhandy::ColumnExt;

#[test]
fn check_build() {
    assert!(gtk::init().is_ok());
    let column = libhandy::Column::new();
    column.set_maximum_width(800);
    column.set_linear_growth_width(600);
}

