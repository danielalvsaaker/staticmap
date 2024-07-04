use staticmap::{
    tools::{Color, RectBuilder},
    Error, StaticMapBuilder,
};

fn main() -> Result<(), Error> {
    let mut map = StaticMapBuilder::new().width(200).height(200).build()?;

    let rect1 = RectBuilder::new()
        .north_lat_coordinate(43.12398687511079)
        .south_lat_coordinate(43.107942538441854)
        .east_lon_coordinate(141.39078581150866)
        .west_lon_coordinate(141.37070467336105)
        .color(Color::new(true, 255, 0, 0, 255))
        .build()?;

    let rect2 = RectBuilder::new()
        .north_lat_coordinate(42.81587629948163)
        .south_lat_coordinate(42.76094473505349)
        .east_lon_coordinate(141.698469171195)
        .west_lon_coordinate(141.65625418708007)
        .color(Color::new(true, 0, 0, 255, 255))
        .stroke_width(2.0)
        .build()?;

    map.add_tool(rect1);
    map.add_tool(rect2);

    map.save_png("rect.png")?;

    Ok(())
}
