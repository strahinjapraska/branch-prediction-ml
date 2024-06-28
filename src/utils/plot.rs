use plotters::{backend::BitMapBackend, chart::ChartBuilder, drawing::IntoDrawingArea, element::{PathElement, Rectangle}, series::LineSeries, style::{full_palette::LIGHTBLUE, RED, BLUE, GREEN, MAGENTA, CYAN, BLACK,  Color, IntoFont, WHITE}};
use std::{collections::HashMap, error::Error};

pub fn plot_histogram(data: Vec<f64>) -> Result<(), Box<dyn Error>> {
    let out_file_name = "data/histogram.png";
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

pub fn plot_series(data: HashMap<&str, Vec<u32>>) -> Result<(), Box<dyn Error>>{
    
    let root = BitMapBackend::new("data/series.png", (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    // Determine the range of x-axis and y-axis based on your data
    let x_max = data.values().map(|v| v.len()).max().unwrap_or(0);
    let y_min = data.values().flat_map(|v| v.iter()).min().cloned().unwrap_or(0);
    let y_max = data.values().flat_map(|v| v.iter()).max().cloned().unwrap_or(0);

    let mut chart = ChartBuilder::on(&root)
        .caption("Mispredictions through time", ("sans-serif", 50).into_font())
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0..x_max, y_min..y_max)?;

    chart.configure_mesh().draw()?; 

    // Define colors
    let colors = vec![&RED, &BLUE, &GREEN, &MAGENTA, &CYAN, &BLACK];

    for (i, (name, series)) in data.iter().enumerate() {
        let color = colors[i % colors.len()]; 
        chart
            .draw_series(LineSeries::new(
                (0..series.len()).map(|x| (x, series[x])),
                color,
            ))?
            .label(*name)
            .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color));
    }

    chart.configure_series_labels()
        .border_style(&BLACK)
        .background_style(&WHITE.mix(0.8))
        .draw()?;

    root.present()?;
    Ok(())
}
