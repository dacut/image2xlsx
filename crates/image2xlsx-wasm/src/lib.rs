use {js_sys::Uint8ClampedArray, wasm_bindgen::prelude::*};
use rust_xlsxwriter::{Workbook, Worksheet, Format};
use rust_xlsxwriter::Color as ExcelColor;

#[wasm_bindgen]
pub fn convert_image_to_excel(image_data: Uint8ClampedArray, width: u32, height: u32) -> Vec<u8> {
    set_panic_hook();

    if image_data.length() < width * height * 4 {
        panic!("Image data length does not match the specified width and height");
    }

    let mut workbook: Workbook = Workbook::new();
    let worksheet: &mut Worksheet = workbook.add_worksheet();

    worksheet.set_default_row_height_pixels(5);
    for x in 0..width {
        worksheet.set_column_width_pixels(x as u16, 10).unwrap();
    }

    // Convert Uint8ClampedArray to Vec<u8>
    let mut index = 0;
    for y in 0..height {
        for x in 0..width {
            let r = image_data.get_index(index);
            let g = image_data.get_index(index + 1);
            let b = image_data.get_index(index + 2);

            index += 4;

            let xl_color = ExcelColor::from((r as u32) << 16 | (g as u32) << 8 | b as u32);
            let format = Format::new().set_background_color(xl_color);
            worksheet.set_cell_format(y, x as u16, &format).unwrap();
        }
    }

    workbook.save_to_buffer().unwrap()
}

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
