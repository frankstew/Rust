use shared;








fn main() {
    shared::print_type_of(&shared::get_lines("./input.txt"));
    println!("{}", shared::get_lines("./input.txt").len());
}
