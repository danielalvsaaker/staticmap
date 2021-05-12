use staticmap::{
    tools::{CircleBuilder, Color},
    Error, StaticMapBuilder,
};

fn main() -> Result<(), Error> {
    let mut map = StaticMapBuilder::new()
        .width(200)
        .height(200)
        .url_template("https://a.tile.osm.org/{z}/{x}/{y}.png")
        .zoom(5)
        .build()?;

    let circle_outline = CircleBuilder::new()
        .lon_coordinate(10.)
        .lat_coordinate(47.)
        .color(Color::new(true, 255, 255, 255, 255))
        .radius(9.)
        .build()?;

    let circle = CircleBuilder::new()
        .lon_coordinate(10.)
        .lat_coordinate(47.)
        .color(Color::new(true, 0, 0, 255, 255))
        .radius(6.)
        .build()?;

    map.add_tool(circle_outline);
    map.add_tool(circle);

    map.save_png("circle.png")?;

    Ok(())
}
