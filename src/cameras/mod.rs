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
            left: pt.x - SCREEN_WIDTH / 2,
            right: pt.x + SCREEN_WIDTH / 2,
            top: pt.y - SCREEN_HEIGHT / 2,
            bottom: pt.y + SCREEN_HEIGHT / 2,
        }
    }

    pub fn on_player_move(&mut self, pt: Point) {
        self.left = pt.x - SCREEN_WIDTH / 2;
        self.right = pt.x + SCREEN_WIDTH / 2;
        self.top = pt.y - SCREEN_HEIGHT / 2;
        self.bottom = pt.y + SCREEN_HEIGHT / 2;
    }

    pub fn offset(&self) -> Point { Point::constant(self.left, self.top) }

    pub fn extent(&self) -> Point { Point::constant(self.right, self.bottom) }

    pub fn center(&self) -> Point { Point::constant(self.left + SCREEN_WIDTH / 2, self.top + SCREEN_HEIGHT + 2) }

    pub fn in_view(&self, pt: Point) -> bool {
        pt.x >= self.left && pt.x < self.right
            && pt.y >= self.top && pt.y < self.bottom
    }

    pub fn in_central_view(&self, pt: Point) -> bool {
        let w = SCREEN_WIDTH / 4;
        let h = SCREEN_HEIGHT / 4;
        pt.x >= self.left + w && pt.x < self.right - w
            && pt.y >= self.top + h && pt.y < self.bottom - h
    }
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
        camera.on_player_move(Point::new(SCREEN_WIDTH, SCREEN_HEIGHT));
        assert_eq!(camera.left, SCREEN_WIDTH / 2);
        assert_eq!(camera.top, SCREEN_HEIGHT / 2);
    }
}