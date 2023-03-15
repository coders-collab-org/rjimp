use rjimp::{plug, prelude::*, Color};

#[tokio::main]
async fn main() {
    let mut img = PNG::new("/home/darky/Pictures/sasuki.png").await.unwrap();

    plug!(img.circle());

    // or
    img.plugin(flip, Default::default());

    // you can use your custom plugin also by:
    // fn custom(_options: (), bitmap: Bitmap) { ... }
    // rjimp::c_plug!(img.custom());

    // edit on pixels by yourself
    let w = img.width();
    let h = img.height();
    let mut bitmap = img.bitmap();

    // get last 50 pixels
    for (_x, _y, idx) in bitmap.scan(w - 50, h - 50) {
        bitmap.set_pixel_by_index(
            idx,
            match idx % 300 {
                0..=99 => Color::red(),
                100..=199 => Color::green(),
                200..=299 => Color::blue(),
                _ => unreachable!(),
            },
        );
    }

    plug!(img.circle());

    img.write("/home/darky/Pictures/sasuki-c.png").unwrap();

    // or
    let data: String = img.get_buffer64().unwrap();

    println!("{data}");
}
