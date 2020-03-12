use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Blob, ImageBitmap, Request, RequestInit, RequestMode, Response};

pub async fn get_image(
    col: u32,
    row: u32,
    url: &str,
    filename: &str,
) -> Result<ImageBitmap, JsValue> {
    let window = web_sys::window().unwrap();
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);
    let request = Request::new_with_str_and_init(
        &format!("{}imgs/{}_{}_{}.png", url, filename, row + 1, col + 1),
        &opts,
    )
    .unwrap();
    let request_promise = window.fetch_with_request(&request);

    let resp_value = JsFuture::from(request_promise).await?;
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    // blobメソッドはResultを返すので?付けてしまう
    let blob_value = JsFuture::from(resp.blob()?).await?;
    assert!(blob_value.is_instance_of::<Blob>());
    let blob: Blob = blob_value.dyn_into().unwrap();

    let bitmap_value = JsFuture::from(window.create_image_bitmap_with_blob(&blob)?).await?;
    assert!(bitmap_value.is_instance_of::<ImageBitmap>());
    let bitmap: ImageBitmap = bitmap_value.dyn_into().unwrap();
    Ok(bitmap)
}
