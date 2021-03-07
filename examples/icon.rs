use staticmap::{tools::IconBuilder, StaticMapBuilder, StaticMapError};

fn main() -> Result<(), StaticMapError> {
    let mut map = StaticMapBuilder::default()
        .width(200)
        .height(200)
        .padding((80, 0))
        .url_template("https://a.tile.osm.org/{z}/{x}/{y}.png")
        .zoom(12)
        .build()
        .unwrap();

    let icon_flag = IconBuilder::default()
        .lon_coordinate(6.63204)
        .lat_coordinate(45.85378)
        .x_offset(12.)
        .y_offset(32.)
        .image("examples/icons/icon-flag.png")
        .build()
        .unwrap();

    let icon_factory = IconBuilder::default()
        .lon_coordinate(6.6015)
        .lat_coordinate(45.8485)
        .x_offset(18.)
        .y_offset(18.)
        .image("examples/icons/icon-factory.png")
        .build()
        .unwrap();

    map.add_marker(icon_flag);
    map.add_marker(icon_factory);

    map.save_png("icon.png")?;

    Ok(())
}
