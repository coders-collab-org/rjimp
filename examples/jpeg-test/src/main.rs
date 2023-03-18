use rjimp::{plug, prelude::*};

#[tokio::main]
async fn main() {
    // Load an image from disk.
    let mut img = JPEG::new("/home/darky/Pictures/itachi.jpg").await.unwrap();

    // plug!(img.circle());

    img.write("/home/darky/Pictures/itachi-c.jpg").unwrap();
}
