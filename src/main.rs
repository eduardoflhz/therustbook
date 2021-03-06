mod fearless_concurrency;
mod data_types;
mod closures;
mod char_count;
mod fibonacci;
mod generics;
mod guessing_game;
mod hash_map;
mod lifetime;
mod iterators;
mod test;
mod rectangles;
mod structs;
mod temperature_converter;
mod slices;
mod references;
mod hello_macro;
mod oop;
mod restaurant;
mod smart_pointers;
mod calculator;

fn main() {
    let fib_nums = vec![0,1];
    fibonacci::generate_sequence(fib_nums, 5);
    closures::closures();
    guessing_game::guess();
    char_count::count_chars("abradacabra");
    fearless_concurrency::call_threads();
    hash_map::employee::test_employee();
    hash_map::grades::test_grades();
    hash_map::pig_latin::test_transform();
    generics::get_largest();
    lifetime::test_lifetime();
    iterators::iter();
    rectangles::test_rectangles();
    structs::test_structs();
    temperature_converter::test_temperatures();
    slices::test_slices();
    references::test_references();
    hello_macro::test_macros();
    oop::test_oop();
    restaurant::serve();
    smart_pointers::test_smrt_ptrs();
    calculator::start();
}
