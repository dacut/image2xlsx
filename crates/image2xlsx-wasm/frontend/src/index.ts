let wasm: any;

async function loadWasm() {
  try {
    wasm = await import("../../pkg/image2xlsx_wasm");
    console.log('WASM loaded successfully');
  } catch (error) {
    console.error('WASM import failed:', error);
  }
}

// Load WASM module
loadWasm();

document.addEventListener('DOMContentLoaded', () => {
  const fileInput = document.getElementById('fileInput') as HTMLInputElement;
  const uploadArea = document.querySelector('.upload-area') as HTMLElement;
  const previewSection = document.getElementById('previewSection') as HTMLElement;
  const imagePreview = document.getElementById('imagePreview') as HTMLImageElement;
  const imageInfo = document.getElementById('imageInfo') as HTMLElement;
  const convertBtn = document.getElementById('convertBtn') as HTMLButtonElement;
  const progressSection = document.getElementById('progressSection') as HTMLElement;
  // const progressFill = document.getElementById('progressFill') as HTMLElement;
  // const statusText = document.getElementById('statusText') as HTMLElement;
  const errorMessage = document.getElementById('errorMessage') as HTMLElement;
  const successMessage = document.getElementById('successMessage') as HTMLElement;
  const canvas = document.getElementById('canvas') as HTMLCanvasElement;
  const ctx = canvas.getContext('2d') as CanvasRenderingContext2D;

  let currentImage: HTMLImageElement | null = null;
  let currentFilename: string | null = null;

  // Drag and drop functionality
  uploadArea.addEventListener('dragover', (e: DragEvent) => {
    e.preventDefault();
    uploadArea.classList.add('dragover');
  });

  uploadArea.addEventListener('dragleave', () => {
    uploadArea.classList.remove('dragover');
  });

  uploadArea.addEventListener('drop', (e: DragEvent) => {
    e.preventDefault();
    uploadArea.classList.remove('dragover');
    const files = e.dataTransfer?.files;
    if (files && files.length > 0) {
      handleFile(files[0]);
    }
  });

  fileInput.addEventListener('change', (e: Event) => {
    const target = e.target as HTMLInputElement;
    if (target.files && target.files.length > 0) {
      handleFile(target.files[0]);
    }
  });

  // Paste handler for images
  document.addEventListener('paste', (e: ClipboardEvent) => {
    const items = e.clipboardData?.items;
    if (!items) return;

    for (let i = 0; i < items.length; i++) {
      const item = items[i];
      
      // Check if the item is an image
      if (item.type.indexOf('image') === 0) {
        e.preventDefault();
        const file = item.getAsFile();
        if (file) {
          handleFile(file);
          showSuccess('Image pasted successfully!');
        }
        break;
      }
    }
  });

  function showError(message: string): void {
    errorMessage.textContent = message;
    errorMessage.style.display = 'block';
    successMessage.style.display = 'none';
    setTimeout(() => {
      errorMessage.style.display = 'none';
    }, 5000);
  }

  function showSuccess(message: string): void {
    successMessage.textContent = message;
    successMessage.style.display = 'block';
    errorMessage.style.display = 'none';
    setTimeout(() => {
      successMessage.style.display = 'none';
    }, 5000);
  }

  function getExcelFilename(originalFilename: string | null): string {
    if (!originalFilename) return 'image.xlsx';

    // Remove the extension and add .xlsx
    const nameWithoutExt = originalFilename.replace(/\.[^/.]+$/, '');
    return nameWithoutExt + '.xlsx';
  }

  function handleFile(file: File): void {
    // Validate file type
    if (!file.type.match(/^image\/.*$/)) {
      showError('Please select an image file. (' + file.type + ' not supported)');
      return;
    }

    // Store the filename for later use
    currentFilename = file.name;

    const reader = new FileReader();
    reader.onload = (e: ProgressEvent<FileReader>) => {
      const img = new Image();
      img.onload = () => {
        currentImage = img;
        imagePreview.src = e.target?.result as string;
        imageInfo.textContent = `File: ${file.name} • Dimensions: ${img.width} × ${img.height} pixels`;
        previewSection.style.display = 'block';
        convertBtn.disabled = false;
      };
      img.src = e.target?.result as string;
    };
    reader.readAsDataURL(file);
  }

  convertBtn.addEventListener('click', async () => {
    if (!currentImage) return;

    convertBtn.disabled = true;
    progressSection.style.display = 'block';

    try {
      // Set canvas dimensions to match image
      const width = currentImage.width;
      const height = currentImage.height;
      canvas.width = width;
      canvas.height = height;

      // Draw image to canvas
      ctx.drawImage(currentImage, 0, 0);

      // Get image data
      const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
      const pixels = imageData.data;

      // Convert to Excel format.
      const result = wasm.convert_image_to_excel(pixels, width, height);

      // Create download
      const blob = new Blob([result], { type: 'application/octet-stream' });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = getExcelFilename(currentFilename);
      document.body.appendChild(a);
      a.click();
      document.body.removeChild(a);
      URL.revokeObjectURL(url);

      showSuccess(
        `Excel file created successfully! (${canvas.width}×${canvas.height} cells)`
      );
    } catch (error) {
      console.error('Conversion error:', error);
      showError('An error occurred during conversion. Please try again.');
    } finally {
      convertBtn.disabled = false;
      setTimeout(() => {
        progressSection.style.display = 'none';
      }, 3000);
    }
  });
});

