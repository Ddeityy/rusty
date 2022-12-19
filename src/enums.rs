use colorsys::Rgb;
fn main() {
    let w = Message::Write(String::from("haha, classic smoking cat"));
    let q = Message::Quit;
    let m = Message::Move { x: 32, y: 16 };
    let c = Message::ChangeColor((255.0, 255.0, 255.0));

    w.call();
    m.call();
    c.call();
    q.call();
}

#[derive(Debug)]
enum Message {
    Quit,
    Write(String),
    Move { x: i32, y: i32 },
    ChangeColor((f32, f32, f32)),
}

impl Message {
    fn call(&self) {
        match &self {
            Message::Quit => println!("Quit"),
            Message::Move { x, y } => println!("Move to {x},{y}"),
            Message::Write(x) => println!("{x}"),
            Message::ChangeColor(rgb) => {
                let rgb = Rgb::from(rgb);
                let hex = rgb.to_hex_string();
                println!("Color: {hex}");
            }
        }
    }
}
