use staticmap::{Error, StaticMapBuilder};

fn main() -> Result<(), Error> {
    let mut map = StaticMapBuilder::new()
        .width(300)
        .height(300)
        .zoom(4)
        .lon_center(4.)
        .lat_center(54.)
        .build()?;

    map.save_png("empty_map.png")?;

    Ok(())
}
