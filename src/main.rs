mod tests;
mod enums;
use enums::test::Message;
fn main() {
    enums();
}
fn enums() {
    let w = Message::Write(String::from("haha, classic smoking cat"));
    let q = Message::Quit;
    let m = Message::Move { x: 32, y: 16 };
    let c = Message::ChangeColor((255.0, 255.0, 255.0));

    w.call();
    m.call();
    c.call();
    q.call();
}
