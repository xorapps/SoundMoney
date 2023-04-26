mod qr_services;
pub use qr_services::*;

mod errors;
pub use errors::*;

mod receipt_service;
pub use receipt_service::*;

mod mail_services;
pub use mail_services::*;

#[cfg(test)]
mod try_mail {

    use crate::TemplateBuilder;
    use borsh::BorshSerialize;
    use geyser_service_common::BillableItem;
    use smol::io::BufWriter;
    use xor_mailer_common::Envelope;

    #[test]

    fn main() {
        let mut my_items = Vec::<BillableItem>::new();
        my_items.push(
            BillableItem::new()
                .add_cost("USDC-SPL 2")
                .add_img("http://192.168.41.65:5000/img/espresso.jpg")
                .add_name("Espresso"),
        );

        my_items.push(
            BillableItem::new()
                .add_cost("USDC-SPL 0.88")
                .add_img("http://192.168.41.65:5000/img/doughnut.jpg")
                .add_name("Doughnut"),
        );

        my_items.push(
            BillableItem::new()
                .add_cost("USDC-SPL 1.8")
                .add_img("http://192.168.41.65:5000/img/mocha.jpg")
                .add_name("Mocha"),
        );

        my_items.push(
            BillableItem::new()
                .add_cost("USDC-SPL 0.2")
                .add_img("http://192.168.41.65:5000/img/croissant.jpg")
                .add_name("Croissant"),
        );

        let mut builder = TemplateBuilder::new();
        builder.add_customer("Anatoly").add_date("Wed 1 March 2023").add_tx("5U2NLNdCqS1gpvpzzo2VaExNCLR8KtiLm9HHimAqVhctGzizVDY5rfzTJwJnCU2GKjwmVBMkwT8RcUj7Ac86Xyss")
    .add_items(my_items)
    ;

        let mut envelope = Envelope::new();
        envelope
            .add_recipient(("M81 Network", "support@m81.network"))
            .add_subject("Receipt")
            .add_html_body(&builder.to_template_1().unwrap());

        let stream_data = envelope.try_to_vec().unwrap();

        smol::block_on(async {
            use smol::{io::AsyncWriteExt, net::TcpStream};
            let mut stream = TcpStream::connect("127.0.0.1:6363").await.unwrap();

            let mut buf_writer = BufWriter::with_capacity(4096, &mut stream);
            buf_writer.write_all(&stream_data).await.unwrap();

            buf_writer.flush().await.unwrap();
        })
    }
}
