mod print;
mod vars;
mod data_types;
fn main() {

    data_types::run("Vari");
    data_types::add(3,6);
    data_types::cond();
    println!(" {}", data_types::returnNumber(3))
}
