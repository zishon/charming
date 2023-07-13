use charming::{
    component::{Axis, Grid, Legend, Title, Toolbox, ToolboxFeature, ToolboxFeatureSaveAsImage},
    element::{AreaStyle, AxisType, Emphasis, EmphasisFocus, Tooltip, Trigger},
    series::Line,
    Chart,
};

pub fn chart() -> Chart {
    Chart::new()
        .title(Title::new().text("Stacked Line"))
        .tooltip(Tooltip::new().trigger(Trigger::Axis))
        .legend(Legend::new().data(vec![
            "Email",
            "Union Ads",
            "Video Ads",
            "Direct",
            "Search Engine",
        ]))
        .grid(
            Grid::new()
                .left("3%")
                .right("4%")
                .bottom("3%")
                .contain_label(true),
        )
        .toolbox(
            Toolbox::new()
                .feature(ToolboxFeature::new().save_as_image(ToolboxFeatureSaveAsImage::new())),
        )
        .x_axis(
            Axis::new()
                .type_(AxisType::Category)
                .boundary_gap(false)
                .data(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]),
        )
        .y_axis(Axis::new().type_(AxisType::Value))
        .series(
            Line::new()
                .name("Email")
                .stack("Total")
                .area_style(AreaStyle::new())
                .emphasis(Emphasis::new().focus(EmphasisFocus::Series))
                .data(vec![120, 132, 101, 134, 90, 230, 210]),
        )
        .series(
            Line::new()
                .name("Union Ads")
                .stack("Total")
                .area_style(AreaStyle::new())
                .emphasis(Emphasis::new().focus(EmphasisFocus::Series))
                .data(vec![220, 182, 191, 234, 290, 330, 310]),
        )
        .series(
            Line::new()
                .name("Video Ads")
                .stack("Total")
                .area_style(AreaStyle::new())
                .emphasis(Emphasis::new().focus(EmphasisFocus::Series))
                .data(vec![150, 232, 201, 154, 190, 330, 410]),
        )
        .series(
            Line::new()
                .name("Direct")
                .stack("Total")
                .area_style(AreaStyle::new())
                .emphasis(Emphasis::new().focus(EmphasisFocus::Series))
                .data(vec![320, 332, 301, 334, 390, 330, 320]),
        )
        .series(
            Line::new()
                .name("Search Engine")
                .stack("Total")
                .area_style(AreaStyle::new())
                .emphasis(Emphasis::new().focus(EmphasisFocus::Series))
                .data(vec![820, 932, 901, 934, 1290, 1330, 1320]),
        )
}
