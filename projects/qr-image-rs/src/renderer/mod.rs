// use qrcode_generator::QrCodeEcc;

use qrcode::{Version, EcLevel};

#[test]
fn test(){
    use qrcode::QrCode;
    use image::Luma;

// Encode some data into bits.
    let code = QrCode::with_version(b"01234567",Version::Normal(4), EcLevel::L ).unwrap();

// Render the bits into an image.
    let image = code.render::<Luma<u8>>().build();

// Save the image.
    image.save("./qrcode.png").unwrap();

// You can also render it into a string.
    let string = code.render()
        .light_color(' ')
        .dark_color('#')
        .build();
    println!("{}", string);
}