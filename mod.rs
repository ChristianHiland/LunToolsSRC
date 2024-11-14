
// Libs
use plotters::element::PointCollection; // Import PointCollection
use plotters::prelude::*;
use std::fmt;

// Importing The JSON Funcs.
pub mod json;
// Importing The path Funcs.
pub mod path;
// This is fine.
mod tests;

// Single Non-Speific Funcs.

/// Docstring: Print Func
/// Input: string
/// Inter: string to terminal
pub fn print(text: &str) {
    println!("{}", text);
}

// Testing Plot Uses, Posabily.

// Plot2d Testing Func
pub fn plot2d(save_file: &str, title: &str, font: &str, fontsize: u64) -> Result<(), Box<dyn std::error::Error>> {
    // Create a BitMap backend for the plot
    let root = BitMapBackend::new(save_file, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    // Create a chart context
    let mut chart = ChartBuilder::on(&root)
        .caption(title, (font, fontsize as f64).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-10f32..10f32, -10f32..10f32)?;

    chart.configure_mesh().draw()?;

    // Plot a line 
    // (-10..=10) Means -10 counting up to 10
    //  .map Mean to map a dot based on (x,y)
    // |x| IDK EXT, but it helps counting?
    // (x,y) as x (counting) from x plots on X Plot.
    // (x,y) as y (counting) from x plots on Y Plot.
    chart.draw_series(LineSeries::new(
        (-10..=10).map(|x| (x as f32, x as f32)),
        &RED,
    ))?;
    chart.draw_series(LineSeries::new(
        (-9..=9).map(|x| (x as f32, x as f32)),
        &BLUE,
    ))?;
    // Plot Line (Test User input)
    Ok(())
}
// Plot3d Testing Func
pub fn plot3d(save_file: &str, title: &str, font: &str, fontsize: u64) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(save_file, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("3D Plot", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_3d(-10f32..10f32, -10f32..10f32, -10f32..10f32)?;

    chart.configure_axes().draw()?;

    chart.draw_series(LineSeries::new(
        (-10..=10).map(|x| (x as f32, x as f32, x as f32)),
        &RED,
    ))?;

    Ok(())
}