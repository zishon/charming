use echarts::{
    component::axis,
    series::{bar, Series},
    utility::background_style,
    Chart,
};

fn main() {
    let chart = Chart::new()
        .x_axis(
            axis::Axis::new()
                .type_(axis::AxisType::Category)
                .data(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]),
        )
        .y_axis(axis::Axis::new().type_(axis::AxisType::Value))
        .series(Series::Bar(
            bar::Bar::new()
                .show_background(true)
                .background_style(
                    background_style::BackgroundStyle::new()
                        .color("rgba(180, 180, 180, 0.2)".into()),
                )
                .data(vec![120, 200, 150, 80, 70, 110, 130]),
        ));
    println!("{}", chart.to_string());
}
