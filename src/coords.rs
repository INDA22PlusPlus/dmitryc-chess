#[derive(Debug, PartialEq, Eq)]
pub struct Size {
    pub w:u8,
    pub h:u8,
}

impl Size {
    pub fn new(w:u8, h:u8) -> Size {
        if w > 7 {
            panic!("Width must be between 0 and 7!")
        }
        if h > 7 {
            panic!("Height must be between 0 and 7!")
        }
        Self{w, h}
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Coords {
    pub x:u8,
    pub y:u8,
}

impl Coords {
    pub fn new(x:u8, y:u8) -> Coords {
        if x > 7 {
            panic!("Width must be between 0 and 7!")
        }
        if y > 7 {
            panic!("Height must be between 0 and 7!")
        }
        Self{x, y}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assign_zeros_size() {
        let s = Size::new(0, 0);
        assert_eq!(s.w, 0);
        assert_eq!(s.h, 0);
    }

    #[test]
    fn assign_ones_size() {
        let s = Size::new(1, 1);
        assert_eq!(s.w, 1);
        assert_eq!(s.h, 1);
    }

    #[test]
    fn assign_different_values_size() {
        let s = Size::new(3, 4);
        assert_eq!(s.w, 3);
        assert_eq!(s.h, 4);
    }

    #[test]
    fn assign_limit_values_size() {
        let s = Size::new(7, 7);
        assert_eq!(s.w, 7);
        assert_eq!(s.h, 7);
    }

    //TODO: How to write this type of test

    // #[test]
    // #[should_panic]
    // fn assign_negative_size() {
    //     let s = Size::new(-1, -1);
    // }

    #[test]
    #[should_panic]
    fn assign_over_7_size() {
        let s = Size::new(8, 8);
    }

    #[test]
    fn assign_zeros_coords() {
        let c = Coords::new(0, 0);
        assert_eq!(c.x, 0);
        assert_eq!(c.y, 0);
    }

    #[test]
    fn assign_ones_coords() {
        let c = Coords::new(1, 1);
        assert_eq!(c.x, 1);
        assert_eq!(c.y, 1);
    }

    #[test]
    fn assign_different_values_coords() {
        let c = Coords::new(3, 4);
        assert_eq!(c.x, 3);
        assert_eq!(c.y, 4);
    }

    #[test]
    fn assign_limit_values_coords() {
        let c = Coords::new(7, 7);
        assert_eq!(c.x, 7);
        assert_eq!(c.y, 7);
    }

    //TODO: How to write this type of test

    // #[test]
    // #[should_panic]
    // fn assign_negative_coords() {
    //     let c = Coords::new(-1, -1);
    // }

    #[test]
    #[should_panic]
    fn assign_over_7_coords() {
        let c = Coords::new(8, 8);
    }

}