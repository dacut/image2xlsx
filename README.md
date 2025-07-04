# Convert images to Excel workbooks
This is an application that converts images (PNG, JPEG, non-animated GIF) to Excel workbooks.
Each cell in the workbook is colored to correspond to the pixel value from the image.

There are a few different crates in here:
* `image2xlsx` is a library for performing the conversion.
* `image2xlsx-cli` provides a command-line interface for converting files.
* `image2xlsx-wasm` provides a web application for converting file. All conversion happens in the browser.

A deployment of `image2xlsx-wasm` is available for use at https://image2xlsx.kanga.org/
