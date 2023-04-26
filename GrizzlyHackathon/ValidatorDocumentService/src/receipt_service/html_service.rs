use geyser_service_common::BillableItem;
use percy_dom::prelude::*;

#[derive(Debug, Default)]
pub struct TemplateBuilder {
    customer_name: String,
    date_of_issue: String,
    items: Vec<BillableItem>,
    tx_signature: String,
}

impl TemplateBuilder {
    pub fn new() -> Self {
        TemplateBuilder::default()
    }

    pub fn add_customer(&mut self, name: &str) -> &mut Self {
        self.customer_name = name.to_owned();

        self
    }

    pub fn add_date(&mut self, date: &str) -> &mut Self {
        self.date_of_issue = date.to_owned();

        self
    }

    pub fn add_item(&mut self, item: BillableItem) -> &mut Self {
        self.items.push(item);

        self
    }

    pub fn add_items(&mut self, items: Vec<BillableItem>) -> &mut Self {
        self.items = items;

        self
    }

    pub fn add_tx(&mut self, signature: &str) -> &mut Self {
        self.tx_signature = signature.to_owned();

        self
    }

    pub fn to_template_1(&self) -> crate::DocumentServiceResult<String> {
        let doctype = "<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.0 Transitional//EN\" \"http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd\">";
        let content_type =
            "<meta http-equiv=\"Content-Type\" content=\"text/html; charset=utf-8\" />";
        let http_equiv = "<meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\" />";
        let viewport = "<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">";
        let document_title = "Stellar Saga Coffee";

        let font_url = "@import url('https://fonts.googleapis.com/css2?family=Roboto+Condensed:wght@700&display=swap');";

        use crate::QrGenerator;

        let mut tx_sign_url = String::from("https://explorer.solana.com/tx/");
        tx_sign_url.push_str(&self.tx_signature);

        let qr_base64 = QrGenerator::new()
            .add_border(4)
            .change_error_correction_level(qrcode_generator::QrCodeEcc::High)
            .add_content(&tx_sign_url)
            .build_qr_png()?;

        let items = self
            .items
            .iter()
            .map(|item| {
                html! {
                <tr>
                    <td style="width: 50px" style="text-align: center;">
                        <img src={&item.img().to_owned()} alt="" srcset="" width="100px">
                    </td>
                    <td style="width: 250px; padding: 50px;">
                        <span style=" width: 100px; margin-right: 10px"> {&item.name().to_owned()} </span>
                        <span style="color: rgb(24, 136, 211);  "> {&item.cost().to_owned()} </span>
                    </td>
                </tr>
                }
            })
            .collect::<Vec<VirtualNode>>();

        let style_data = r#"
    /* No spacing of top or left edge */
    body {
        margin: 0;
    }

    /* No spacing between tables */
    table {
        border-spacing: 0;
    }

    td {
        padding: 0;
    }

    img {
        border: 0;
    }

    .width-50 {
        width: 100%;
        max-width: 300px;
        display: inline-block;
        vertical-align: top;
    }

    .wrapper {
        width: 100%;
        table-layout: fixed;
        background-color: #e5dec7;
        padding-bottom: 60px;
        text-align: center;
    }

    .main {
        width: 100%;
        max-width: 600px;
        background-color: #ffffff;
        font-family: sans-serif;
        color: #4a4a4a;
        box-shadow: 0 0 25px rgba(0, 0, 0, .15);
    }

    .social {
        width: 38px;
    }

    .social-media {
        padding: 12px 0 12px 62px;
    }

    .two-columns {
        /* Remove padding for the links*/
        font-size: 0;
        text-align: center;
    }

    .two-columns .column {
        width: 100%;
        max-width: 300px;
        display: inline-block;
        vertical-align: top;
    }

    tr {
        border-bottom: 1px solid red;
    }
    "#;

        let template1 = html! {
        {doctype}
        <html xmlns="http://www.w3.org/1999/xhtml">

            <head>
                { content_type }
                { http_equiv }
                { viewport }
                <title>{document_title}</title>
            </head>

            <style>
            { font_url }
            { style_data }

            </style>
                <body>
                    <div class="wrapper">
                        <table class="main" width="100%">
                            // BORDER
                            <tr>
                                <td height="8" style="background-color: #000000;"></td>
                            </tr>

                            // LOGO & SOCIAL MEDIA SECTION

                            <tr>
                                <td style="padding: 14px 0 4px">
                                    <table width="100%">
                                        <tr>
                                            <td class="two-columns">
                                                <table class="column">
                                                    <tr>
                                                        <td style="padding: 0 22px 10px">
                                                            <a href="m81.network/fusiondrive"><img src="img/coffee-logo.png"
                                                                    alt="FusionDrive Logo" srcset="" title="FusionDrive Logo"
                                                                    width="50px"></a>
                                                        </td>
                                                        <td>
                                                            <p style="font-size: 35px;">Saga Coffee</p>
                                                        </td>
                                                    </tr>
                                                </table>
                                                <table class="column">
                                                    <tr>
                                                        <td class="social-media">
                                                            <a href="#"><img class="social" src="img/instagram.png" alt=""></a>
                                                            <a href="#"><img class="social" src="img/facebook.png" alt=""></a>
                                                            <a href="#"><img class="social" src="img/twitter.png" alt=""></a>
                                                            <a href="#"><img class="social" src="img/youtube.png" alt=""></a>
                                                            <a href="#"><img class="social" src="img/linkedin.png" alt=""></a>
                                                        </td>
                                                    </tr>

                                                </table>
                                            </td>
                                        </tr>
                                    </table>

                                </td>
                            </tr>

                            // BANNER IMAGE

                            <tr>
                                <td style="text-align: center; font-family: sans-serif;">
                                    <h3 style="font-size : 50px"> {&self.customer_name} </h3>
                                    <p style="font-size : 18px; padding: 20px "> {"Receipt from the Saga
                                    Coffee Shop on "} {&self.date_of_issue} </p>
                                </td>
                            </tr>

                            // TITLE, TEXT & BUTTON

                            // items
                            <tr>
                                <td style="padding: 5px;">
                                    <table width="100%">

                                    {  items
                                    }

                                    </table>



                                    // Divider
                                    <table style="width: 600px">
                                    <tr>
                                        <td>
                                            <hr style="width: 100%; border: 0px; border-bottom: 3px rgb(24, 136, 211) dotted;">
                                        </td>
                                    </tr>
                                </table>

                                <table style="max-width: 600px; text-align: center; width: 100%; padding: 10px;">
                                <tr>
                                    <td>
                                        <a style="font-size: 10px"
                                            href={tx_sign_url}> {"Tx Signature:"} {self.tx_signature.to_owned()}
                                        </a>
                                        <div style="width: 20px; border: transparent"> </div>
                                    </td>
                                </tr>
                                <tr>
                                    <td>
                                        <img src={qr_base64} alt="" width="400px">
                                    </td>
                                </tr>
                                <tr style="width: 100%">
                                    <td>
                                        {"Scan this QR code to view the transaction on Solana Chain"}
                                    </td>
                                </tr>
                            </table>
                                </td>
                            </tr>


                        </table>
                    </div>
                </body>
        </html>
        };

        Ok(template1.to_string())
    }
}
