fn main() {
    let text = viewbuilder::view("Hello World!");
    *text.get().borrow_mut() = "";
}
