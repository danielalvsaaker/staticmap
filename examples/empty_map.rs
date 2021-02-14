use staticmap::{StaticMapBuilder, StaticMapError};

fn main() -> Result<(), StaticMapError> {
    let mut map = StaticMapBuilder::default()
        .width(300)
        .height(300)
        .zoom(4)
        .lon_center(4.)
        .lat_center(54.)
        .build()
        .unwrap();

    map.save_png("empty_map.png")?;

    Ok(())
}
