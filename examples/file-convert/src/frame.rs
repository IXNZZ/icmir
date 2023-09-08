use bytes::Bytes;
use keyframe::{AnimationSequence, keyframes};
use keyframe_derive::CanTween;

pub fn test_frame() {
    let start = ActorFrame::new(0.0, 0.0);
    let end = ActorFrame::new(10.0, 20.0);

    let time = 1.0;

    let mut key: AnimationSequence<ActorFrame> = keyframes![(start, 0.0), (end, time)];

    // key.advance_and_maybe_wrap(0.1);
    // let frame = key.now_strict().unwrap();
    // println!("frame x: {}, y: {}", frame.x, frame.y);
    // key.advance_and_maybe_wrap(0.1);
    // let frame = key.now_strict().unwrap();
    // println!("frame x: {}, y: {}", frame.x, frame.y);
    // key.advance_and_maybe_wrap(0.1);
    // let frame = key.now_strict().unwrap();
    // println!("frame x: {}, y: {}", frame.x, frame.y);

    for i in 0..20 {
        key.advance_and_maybe_wrap(0.1);
        let frame = key.now_strict().unwrap();
        println!("frame x: {}, y: {}", frame.x, frame.y);
    }
}

#[derive(CanTween, Clone)]
pub struct ActorFrame {
    x: f32,
    y: f32,
    // image: ActorImage
}

pub struct ActorImage {
    byte: Bytes
}

impl ActorFrame {
    pub fn new(x: f32, y: f32) -> Self {
        Self {x, y}
    }
}

