//! Convert an image to an Excel workbook.

use {
    clap::Parser,
    image2xlsx::{ConvertOptions, convert, image::open as open_image},
    std::path::{Path, PathBuf},
};

#[derive(Debug, Parser)]
#[command(version, about)]
struct Args {
    /// The width of a cell in the Excel worksheet. If unspecified, defaults to 10 pixels.
    #[arg(long, default_value = "10")]
    cell_width: u16,

    /// The height of a cell in the Excel worksheet. If unspecified, defaults to 10 pixels.
    #[arg(long, default_value = "10")]
    cell_height: u16,

    /// The name of the worksheet. If unspecified, defaults to "Sheet1".
    #[arg(long, default_value = "Sheet1")]
    worksheet_name: String,

    /// The input image file path. This is required.
    input_filename: String,

    /// The output Excel file path. If unspecified, defaults to the original image file name
    /// with the extension changed to `.xlsx`.
    #[arg(default_value = Option::None)]
    output_filename: Option<String>,
}

fn main() {
    let args = Args::parse();
    let options = ConvertOptions {
        cell_width: args.cell_width,
        cell_height: args.cell_height,
        worksheet_name: args.worksheet_name,
    };
    
    let input_filename: &Path = Path::new(&args.input_filename);
    let output_filename: PathBuf = args.output_filename.map(PathBuf::from).unwrap_or_else(|| input_filename.with_extension("xlsx"));

    let image = open_image(input_filename)
        .unwrap_or_else(|e| panic!("Failed to open image file {}: {}", input_filename.display(), e))
        .into_rgb8();

    let mut workbook = convert(&image, options)
        .unwrap_or_else(|e| panic!("Failed to convert image to Excel workbook: {e}"));

    workbook.save(&output_filename)
        .unwrap_or_else(|e| panic!("Failed to save Excel workbook to {}: {}", output_filename.display(), e));
}
