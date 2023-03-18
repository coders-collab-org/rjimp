use rjimp::{plug, prelude::*, Color};

#[tokio::main]
async fn main() {
    // Load an image from disk.
    let mut img = JPEG::new("/home/darky/Pictures/itachi.jpg").await.unwrap();

    // Apply a built-in plugin (circle).
    plug!(img.circle());

    // Or, apply a plugin manually.
    img.plugin(flip, Default::default());

    // Create a custom plugin.
    // fn custom(_options: (), bitmap: Bitmap) { ... }
    // rjimp::c_plug!(img.custom());

    // Edit pixels directly.
    let w = img.width();
    let h = img.height();
    let mut bitmap = img.bitmap();

    // Get the last 50 pixels and change their color.
    for (_x, _y, idx) in bitmap.scan(w - 50, h - 50) {
        bitmap.set_pixel_by_index(
            idx,
            match idx % 3000 {
                0..=999 => Color::red(),
                1000..=1999 => Color::green(),
                2000..=2999 => Color::blue(),
                _ => unreachable!(),
            },
        );
    }

    // Apply another built-in plugin (circle).
    plug!(img.circle());

    // Write the modified image to disk.
    img.write("/home/darky/Pictures/itachi-c.jpg").unwrap();

    // Or, get the image buffer as a base64-encoded string.
    let data: String = img.get_buffer64().unwrap();

    println!("{data}");
}
