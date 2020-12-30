use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Camera {
    pub left: i32,
    pub right: i32,
    pub top: i32,
    pub bottom: i32,
}

impl Camera {
    pub fn new(pt: Point) -> Self {
        Self{
            left: pt.x - DISPLAY_WIDTH / 2,
            right: pt.x + DISPLAY_WIDTH / 2,
            top: pt.y - DISPLAY_HEIGHT / 2,
            bottom: pt.y + DISPLAY_HEIGHT / 2,
        }
    }

    pub fn on_player_move(&mut self, pt: Point) {
        self.left = pt.x - DISPLAY_WIDTH / 2;
        self.right = pt.x + DISPLAY_WIDTH / 2;
        self.top = pt.y - DISPLAY_HEIGHT / 2;
        self.bottom = pt.y + DISPLAY_HEIGHT / 2;
    }

    pub fn offset(&self) -> Point { Point::constant(self.left, self.top) }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_camera() {
        let camera = Camera::new(Point::zero());
        assert!(camera.left < 0);
        assert!(camera.right > 0);
        assert!(camera.top < 0);
        assert!(camera.bottom > 0);
    }

    #[test]
    fn test_camera_move() {
        let mut camera = Camera::new(Point::zero());
        camera.on_player_move(Point::new(DISPLAY_WIDTH, DISPLAY_HEIGHT));
        assert_eq!(camera.left, DISPLAY_WIDTH / 2);
        assert_eq!(camera.top, DISPLAY_HEIGHT / 2);
    }
}