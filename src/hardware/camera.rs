use opencv::core::Mat;
use opencv::imgcodecs::imwrite;
use opencv::videoio::{CAP_ANY, VideoCapture, VideoCaptureTrait};

pub fn human_from_camera(path: &str) {
    let mut camera = VideoCapture::new(0, CAP_ANY).unwrap();
    let mut frame = Mat::default();
    camera.read(&mut frame).unwrap();
    let params = opencv::types::VectorOfi32::new();
    imwrite(path, &frame, &params).unwrap();
}