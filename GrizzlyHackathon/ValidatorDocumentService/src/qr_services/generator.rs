use crate::DocumentServiceResult;
use qrcode_generator::QrCodeEcc;

#[derive(Debug)]
pub struct QrGenerator<'qr> {
    content: &'qr str,
    error_correction_level: QrCodeEcc,
    border: i32,
}

impl<'qr> QrGenerator<'qr> {
    pub fn new() -> Self {
        QrGenerator {
            content: "[UNINITIALIZED_CONTENT_FOR_QR_CODE]",
            error_correction_level: QrCodeEcc::High,
            border: i32::default(),
        }
    }

    pub fn add_content(&mut self, content: &'qr str) -> &mut Self {
        self.content = content;

        self
    }

    pub fn change_error_correction_level(&mut self, level: QrCodeEcc) -> &mut Self {
        self.error_correction_level = level;

        self
    }

    pub fn add_border(&mut self, border: u32) -> &mut Self {
        self.border = border as i32;

        self
    }

    pub fn build_qr_png(&self) -> DocumentServiceResult<String> {
        use base64::{engine::general_purpose, Engine as _};

        let qr_data: Vec<u8> =
            qrcode_generator::to_png_to_vec(self.content, QrCodeEcc::High, 1024).unwrap(); //TODO

        let mut base64_img = String::from("data:image/png;base64,");
        base64_img.push_str(general_purpose::STANDARD_NO_PAD.encode(qr_data).as_str());

        Ok(base64_img)
    }
}
