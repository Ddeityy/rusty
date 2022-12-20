pub mod test {
    use colorsys::Rgb;
    pub enum Message {
        Quit,
        Write(String),
        Move { x: i32, y: i32 },
        ChangeColor((f32, f32, f32)),
    }

    impl Message {
        pub fn call(&self) {
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
}
