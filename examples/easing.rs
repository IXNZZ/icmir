use std::time::Instant;
use ggez::glam::vec2;
use keyframe::{AnimationSequence, keyframes};
use keyframe::functions::Linear;
use tracing::warn;

pub fn main() {
    let mut x: AnimationSequence<f32> = keyframes![(0.0, 0.0, Linear), (4.0 - 0.0001, 2.0, Linear)];

    for i in 0..50 {
        let wrap = x.advance_and_maybe_wrap(0.08);

        let y = x.now() as u32;
        // if y == 4 {
        println!("idx: {}, x: {}, finish: {}, wrap: {wrap}, w: {}", i, y, (x.progress() * 100.0).round(), wrap);
        // println!("keyframes: {}, pair: {:?}", x.keyframes(), x.pair());
        // }
    }

    // let src = vec2(0.0, 0.0);
    // let dst = vec2(48.0, 32.0);
    // let instant = Instant::now();
    // // let angle = src.angle_between(dst);
    // println!("time: {:?}", instant.elapsed());
    // let dist = src.distance(dst);
    // println!("glam: {}, time: {:?}", dist, instant.elapsed());
    //
    // let my_dist = my_distance(src.x, src.y, dst.x, dst.y);
    // println!("my_dist: {}, time: {:?}", my_dist, instant.elapsed());
    //
    // let dis = (dst - src).length_squared();
    // println!("dis: {}, time: {:?}", dis, instant.elapsed());

}

pub fn my_distance(src_x: f32, src_y: f32, dst_x: f32, dst_y: f32) -> f32 {
    ((dst_x - src_x).abs().powi(2) + (dst_y - src_y).abs().powi(2)).sqrt()
}


mod easing {
    use keyframe::{AnimationSequence, CanTween, keyframes};

    #[derive(PartialEq, Clone, Copy)]
    pub enum EasingStatus {
        Ready,
        Run,
        Pause,
        PauseStart,
        PauseFinish,
        Stop,
    }

    pub struct Easing<T: CanTween + Clone + Default> {
        status: EasingStatus,
        sequence: AnimationSequence<T>
    }

    impl<T: CanTween + Clone + Default> Easing<T> {
        pub fn new(start: T, finish: T, time: f64) -> Self {
            let sequence = keyframes![
                (start, time, keyframe::functions::Linear),
                (finish, time, keyframe::functions::Linear)
            ];
            Self {
                status: EasingStatus::Ready,
                sequence
            }
        }

        pub fn run(&mut self) {
            self.status = EasingStatus::Run;
        }

        pub fn pause(&mut self) {
            self.status = EasingStatus::Pause;
        }

        pub fn pause_to_start(&mut self) {
            self.status = EasingStatus::PauseStart;
        }

        pub fn pause_to_finish(&mut self) {
            self.status = EasingStatus::PauseFinish;
        }

        pub fn stop(&mut self) {
            self.status = EasingStatus::Stop;
        }

        pub fn ready(&mut self) {
            self.status = EasingStatus::Ready;
            self.sequence.advance_to(0.0);
        }

        pub fn advance_warp(&mut self, duration: f64) {
            if self.status == EasingStatus::Run {
                self.sequence.advance_and_maybe_wrap(duration);
            } else if self.status == EasingStatus::PauseStart && self.sequence.advance_and_maybe_wrap(duration) {
                self.pause();
            } else if self.status == EasingStatus::PauseFinish {
                if self.sequence.advance_by(duration) > 0.0 {
                    self.sequence.advance_to(0.0);
                    self.pause();
                }
            }
        }

        pub fn now(&self) -> T {
            self.sequence.now()
        }

        pub fn status(&self) -> EasingStatus {
            self.status
        }
    }

    fn angle(src_x: f32, src_y: f32, dst_x: f32, dst_y: f32) -> f32 {
        (dst_y - src_y).atan2(dst_x - src_x) * 180.0 / std::f32::consts::PI
    }

    fn sharing(angle: f32, sharing: u32) -> f32 {
        let sub = 90.0 + 360.0 / 2.0 / sharing as f32 + angle;
        let angle = if sub < 0.0 { sub + 360.0 } else { sub };
        let sub = 360.0 / sharing as f32;
        for s in 0..sharing {
            if angle >= s as f32 * sub && angle < (s as f32 + 1.0) * sub {
                return s as f32 + 1.0;
            }
        }
        return sub;
    }

    pub fn angle8(src_x: f32, src_y: f32, dst_x: f32, dst_y: f32) -> f32 {
        // let angle = angle(src, dst);
        return sharing(angle(src_x, src_y, dst_x, dst_y), 8);
    }

    pub fn angle12(src_x: f32, src_y: f32, dst_x: f32, dst_y: f32) -> f32 {
        return sharing(angle(src_x, src_y, dst_x, dst_y), 12);
    }

    pub fn angle16(src_x: f32, src_y: f32, dst_x: f32, dst_y: f32) -> f32 {
        return sharing(angle(src_x, src_y, dst_x, dst_y), 16);
    }

    pub fn distance(src_x: f32, src_y: f32, dst_x: f32, dst_y: f32) -> f32 {
        ((dst_x - src_x).abs().powi(2) + (dst_y - src_y).abs().powi(2)).sqrt()
    }
}