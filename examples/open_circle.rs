use staticmap::{
    tools::{CircleBuilder, Color},
    Error, StaticMapBuilder,
};

fn main() -> Result<(), Error> {
    let mut map = StaticMapBuilder::new().width(200).height(200).build()?;

    let circle = CircleBuilder::new()
        .lon_coordinate(-3.17)
        .lat_coordinate(55.98)
        .color(Color::new(true, 255, 0, 0, 255))
        .stroke_width(2.);

    map.add_tool(circle.clone().radius_in_meters(500.0 * 1609.34).build()?);

    map.add_tool(circle.clone().radius_in_meters(1000.0 * 1609.34).build()?);

    map.save_png("open_circle.png")?;

    Ok(())
}
