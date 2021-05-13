use staticmap::{tools::IconBuilder, Error, StaticMapBuilder};

fn main() -> Result<(), Error> {
    let mut map = StaticMapBuilder::new()
        .width(200)
        .height(200)
        .padding((80, 0))
        .url_template("https://a.tile.osm.org/{z}/{x}/{y}.png")
        .zoom(12)
        .build()?;

    let icon_flag = IconBuilder::new()
        .lon_coordinate(6.63204)
        .lat_coordinate(45.85378)
        .x_offset(12.)
        .y_offset(32.)
        .path("examples/icons/icon-flag.png")?
        .build()?;

    let icon_factory = IconBuilder::new()
        .lon_coordinate(6.6015)
        .lat_coordinate(45.8485)
        .x_offset(18.)
        .y_offset(18.)
        .path("examples/icons/icon-factory.png")?
        .build()?;

    map.add_tool(icon_flag);
    map.add_tool(icon_factory);

    map.save_png("icon.png")?;

    Ok(())
}
