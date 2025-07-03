use {
    anyhow::{Result, anyhow},
    image::RgbImage,
    rust_xlsxwriter::{Color as ExcelColor, Format, Workbook, Worksheet},
};

pub use {image, rust_xlsxwriter};

/// Options for converting an image to Excel workbook format.
#[derive(Clone, Debug)]
pub struct ConvertOptions {
    /// The width of a cell in the Excel worksheet.
    pub cell_width: u16,

    /// The height of a cell in the Excel worksheet.
    pub cell_height: u16,

    /// The name of the worksheet.
    pub worksheet_name: String,
}

impl Default for ConvertOptions {
    fn default() -> Self {
        ConvertOptions {
            cell_width: 10,
            cell_height: 10,
            worksheet_name: "Sheet1".to_string(), // Default worksheet name
        }
    }
}

/// Convert an image to Excel workbook format using the default options.
pub fn convert_default(image: &RgbImage) -> Result<Workbook> {
    convert(image, ConvertOptions::default())
}

/// Convert an image to Excel workbook format.
pub fn convert(image: &RgbImage, options: ConvertOptions) -> Result<Workbook> {
    let width = image.width();
    let height = image.height();

    if width > u16::MAX as u32 || height > u16::MAX as u32 {
        return Err(anyhow!("Image dimensions exceed maximum allowed size"));
    }

    let mut workbook: Workbook = Workbook::new();
    let worksheet: &mut Worksheet = workbook.add_worksheet();

    if options.cell_width == 0 {
        return Err(anyhow!("Cell width must be greater than zero"));
    }

    for x in 0..width {
        worksheet.set_column_width_pixels(x as u16, options.cell_width)?;
    }

    if options.cell_height == 0 {
        return Err(anyhow!("Cell height must be greater than zero"));
    }

    worksheet.set_default_row_height_pixels(options.cell_height);
    worksheet.set_name(options.worksheet_name)?;

    for (row, row_pixels) in image.rows().enumerate() {
        for (col, pixel) in row_pixels.enumerate() {
            let col = col as u16;
            let row = row as u32;
            let red = pixel[0];
            let green = pixel[1];
            let blue = pixel[2];

            let xl_color = ExcelColor::from((red as u32) << 16 | (green as u32) << 8 | blue as u32);
            let format = Format::new().set_background_color(xl_color);
            worksheet.set_cell_format(row, col, &format)?;
        }
    }

    Ok(workbook)
}
