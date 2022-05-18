use derive_more::From;
use image;
use reqwest;
use std::io::{Cursor, Read, Seek, SeekFrom};
use std::time::Instant;
use timing::measure_duration;

#[macro_use]
extern crate rocket;

const CAT_URL: &'static str = "https://images.unsplash.com/photo-1611915387288-fd8d2f5f928b?ixlib=rb-1.2.1&ixid=MnwxMjA3fDB8MHxzZWFyY2h8MXx8Y2F0JTIwZXllc3xlbnwwfHwwfHw%3D&w=1000&q=80";

#[derive(From, Debug)]
enum GetImageError {
    ImagesError(image::ImageError),
    ReqwestError(reqwest::Error),
}

async fn get_image<'r>(url: &str) -> Result<image::DynamicImage, GetImageError> {
    let start = Instant::now();
    let bytes = reqwest::get(url).await?.bytes().await?;
    println!("Download duration: {:?}", start.elapsed());

    Ok(image::load_from_memory(&bytes)?)
}

#[measure_duration]
fn image_to_png(image: image::DynamicImage) -> Vec<u8> {
    let mut buf = Cursor::new(Vec::new());

    image
        .write_to(&mut buf, image::ImageOutputFormat::Png)
        .expect("An error ocurred!");

    buf.seek(SeekFrom::Start(0)).expect("An error ocurred");

    let mut out = Vec::new();
    buf.read_to_end(&mut out)
        .expect("An error ocurred copying the cursor");

    return out;
}

#[get("/")]
fn index() -> &'static str {
    "Hello world"
}

#[derive(Responder, Debug, Clone)]
enum CatResponses<'r> {
    #[response(status = 200, content_type = "image/png")]
    ImageRes(Vec<u8>),
    #[response(status = 500)]
    Failure(&'r str),
}

#[get("/cat")]
async fn cat_endpoint<'a>() -> CatResponses<'a> {
    let start = Instant::now();
    if let Ok(cat) = get_image(CAT_URL).await {
        let res = CatResponses::ImageRes(image_to_png(cat));
        println!("Request duration: {:?}", start.elapsed());
        res
    } else {
        return CatResponses::Failure("An error ocurred");
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![cat_endpoint])
}
