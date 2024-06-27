use plotters::{backend::BitMapBackend, chart::ChartBuilder, drawing::IntoDrawingArea, element::Rectangle, style::{full_palette::LIGHTBLUE, Color, WHITE}};
use std::error::Error;

pub fn plot_results(data: Vec<f64>) -> Result<(), Box<dyn Error>> {
    let out_file_name = "histogram.png";
    let root = BitMapBackend::new(out_file_name, (1280, 720)).into_drawing_area();

    root.fill(&WHITE)?;

    let max_y = *data.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(&0.0) + 1.0;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(50)
        .y_label_area_size(50)
        .margin(10)
        .caption("Prediction rates", ("sans-serif", 30))
        .build_cartesian_2d(-0.5..(data.len() as f64 - 0.5), 0.0..max_y)?;

    chart.configure_mesh()
        .x_labels(0) 
        .y_labels(10)
        .draw()?;

    let bar_width = 0.5; 

    chart.draw_series(
        data.iter().enumerate().map(|(x, &y)| {
            Rectangle::new(
                [(x as f64 - bar_width / 2.0, 0.0), (x as f64 + bar_width / 2.0, y)],
                LIGHTBLUE.filled(),
            )
        })
    )?;

    root.present()?;
    println!("Result has been saved to {}", out_file_name);

    Ok(())
}
