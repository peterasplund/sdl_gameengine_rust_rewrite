extern crate sdl2;

use std::vec;

struct Rect {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

struct Frame {
    rect: Rect,
    time_to_next_frame: u32,
}

struct Animation {
    active: bool,
    looping: bool,
    frames: Vec<Frame>,
    ellapsed_ms: u32,
    current_frame: usize,
}

impl Animation {
    fn advance_frame(&mut self) {
        if !self.active {
            return;
        }

        if self.current_frame >= self.frames.len() - 1 {
            if self.looping {
                self.current_frame = 0;
            }
        }
        else {
            self.current_frame += 1;
        }
    }

    // @TODO: add positions from current frame position
    fn get_texture_rect(&self) -> Rect {
        let active_frame = self.frames.get(self.current_frame).expect("Current frame is out of bounds").rect;

        Rect {
            x: active_frame.x,
            y: active_frame.y,
            width: active_frame.width,
            height: active_frame.height,
        }
    }
}

impl Default for Animation {
    fn default() -> Animation {
        Animation {
            active: true,
            looping: true,
            ellapsed_ms: 0,
            current_frame: 0,
            frames: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::animation::{Frame,Rect, Animation};

    fn create_mock_animation(looping: bool, active: bool) -> Animation {
        let frame1 = Frame { rect: Rect { x: 0, y: 0, width: 32, height: 32 }, time_to_next_frame: 200 };
        let frame2 = Frame { rect: Rect { x: 32, y: 0, width: 32, height: 32 }, time_to_next_frame: 200 };

        Animation {
            frames: vec![frame1, frame2],
            active,
            looping,
            ellapsed_ms: 0,
            current_frame: 0,
        }
    }

    #[test]
    fn advance_frame_should_not_loop_around_when_not_looping() {
        let mut animation = create_mock_animation(false, true);

        assert_eq!(animation.current_frame, 0);
        animation.advance_frame();
        assert_eq!(animation.current_frame, 1);
        animation.advance_frame();
        assert_eq!(animation.current_frame, 1);
    }

    #[test]
    fn advance_frame_should_loop_around_when_looping() {
        let mut animation = create_mock_animation(true, true);

        assert_eq!(animation.current_frame, 0);
        animation.advance_frame();
        assert_eq!(animation.current_frame, 1);
        animation.advance_frame();
        assert_eq!(animation.current_frame, 0);
    }

    #[test]
    fn advance_frame_should_not_do_anythin_when_not_active() {
        let mut animation = create_mock_animation(true, false);

        assert_eq!(animation.current_frame, 0);
        animation.advance_frame();
        assert_eq!(animation.current_frame, 0);
    }

    #[test]
    fn advance_frame_should_shift_rect() {
        let mut animation = create_mock_animation(true, true);

        assert_eq!(animation.frames.get(animation.current_frame).unwrap().rect.x, 0);
        animation.advance_frame();
        assert_eq!(animation.frames.get(animation.current_frame).unwrap().rect.x, 1);
    }
}

