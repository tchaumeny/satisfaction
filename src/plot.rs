use plotters::prelude::*;
use slugify::slugify;


pub fn plot_series(title: String, alphas: Vec<f32>, results: Vec<f32>) -> Result<String, Box<dyn std::error::Error>> {
    let slug = slugify!(&title);
    let path = format!("{}.png", slug);
    let root = BitMapBackend::new(&path, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 30).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(40)
        .build_cartesian_2d(alphas[0]..alphas[alphas.len() - 1], 0f32..1f32)?;

    chart
        .configure_mesh()
        .x_desc("α (ratio of clauses to variables)")
        .y_desc("Share of satisfiable formulas")
        .draw()?;

    chart
        .draw_series(LineSeries::new(
            alphas.into_iter().zip(results.into_iter()),
            Into::<ShapeStyle>::into(&RED).stroke_width(2),
        ))?;

    root.present()?;

    Ok(path.clone())
}
