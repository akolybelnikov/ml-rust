use gnuplot::{AxesCommon, Caption, Figure, Graph};
use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let mut x: Vec<f64> = Vec::new();
    let mut y: Vec<f64> = Vec::new();
    //TODO: red the CSV data into x and y
    let mut reader = csv::Reader::from_reader(io::stdin());
    for result in reader.records() {
        let record = result?;
        x.push(record[0].parse::<f64>()?);
        y.push(record[1].parse::<f64>()?);
    }
    let mut fg = Figure::new();
    fg.axes2d()
        .set_title("Cat body measurements", &[])
        .set_legend(Graph(0.9), Graph(0.1), &[], &[])
        .set_x_label("Height (cm)", &[])
        .set_y_label("Length (cm)", &[])
        .points(x, y, &[Caption("Cat")]);
    fg.show().expect("TODO: panic message");
    Ok(())
}
