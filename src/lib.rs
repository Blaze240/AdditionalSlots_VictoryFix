#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    unused_imports,
	unused_macros,
	unused_variables,
	unused_assignments,
	unused_unsafe,
	non_upper_case_globals,
	non_snake_case,
    clippy::borrow_interior_mutable_const
)]

mod byleth;
mod corrin;
mod robin;
mod ike;

mod ptrainer_win1;
mod ptrainer_win2;
mod ptrainer_win3;

mod wft_win1;
mod wft_win2;
mod wft_win3;

#[skyline::main(name = "additional_slots_victoryfix_SL2")]
pub fn main() {
    byleth::install();
    corrin::install();
    robin::install();
    ike::install();

    ptrainer_win1::install();
    ptrainer_win2::install();
    ptrainer_win3::install();

    wft_win1::install();
    wft_win2::install();
    wft_win3::install();
}