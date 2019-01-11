//! Basic example of rendering on Cairo.

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlCanvasElement};

use piet::RenderContext;
use piet_web::WebRenderContext;

use piet_test::draw_test_picture;

#[wasm_bindgen]
pub fn run() {
    let canvas = window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();
    let mut context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let dpr = window().unwrap().device_pixel_ratio();
    canvas.set_width((canvas.offset_width() as f64 * dpr) as u32);
    canvas.set_height((canvas.offset_height() as f64 * dpr) as u32);
    let _ = context.scale(dpr, dpr);

    let mut piet_context = WebRenderContext::new(&mut context);
    draw_test_picture(&mut piet_context, 0);
    piet_context.finish();
}
