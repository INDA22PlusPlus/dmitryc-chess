#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Size {
    pub w:usize,
    pub h:usize,
}

impl Size {
    pub fn new(w:usize, h:usize) -> Size {
        if Self::check_width(w) {
            panic!("Width must be between 1 and 8!")
        }
        if Self::check_height(h) {
            panic!("Height must be between 1 and 8!")
        }
        Self{w, h}
    }

    pub fn check_width(w:usize) -> bool {
        w == 0 || w > 8
    }

    pub fn check_height(h:usize) -> bool {
        h == 0 || h > 8
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct RelCoords {
    pub x:isize,
    pub y:isize,
}

impl RelCoords {
    pub fn new(x:isize, y:isize) -> RelCoords{
        RelCoords{x, y}
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Coords {
    pub x:usize,
    pub y:usize,
}

impl Coords {
    pub fn new(x:usize, y:usize) -> Coords {
        if x > 7 {
            panic!("Width must be between 0 and 7!")
        }
        if y > 7 {
            panic!("Height must be between 0 and 7!")
        }
        Self{x, y}
    }

    pub fn check_within_coords(coords: Coords, rel_coords: RelCoords) -> bool {
        (coords.x as isize + rel_coords.x >= 0 && coords.x as isize + rel_coords.x <= 7) &&
            (coords.y as isize + rel_coords.y >= 0 && coords.y as isize + rel_coords.y <= 7)
    }

    pub fn coords_and_rel_coords_result(coords: Coords, rel_coords: RelCoords) -> Coords {
        Coords::new((rel_coords.x + coords.x as isize) as usize,
                    (rel_coords.y + coords.y as isize) as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn assign_zeros_size() {
        Size::new(0, 0);
    }

    #[test]
    #[should_panic]
    fn assign_zero_width_size() {
        Size::new(0, 1);
    }

    #[test]
    #[should_panic]
    fn assign_zero_height_size() {
         Size::new(1, 0);
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
        let s = Size::new(8, 8);
        assert_eq!(s.w, 8);
        assert_eq!(s.h, 8);
    }

    //TODO: How to write this type of test

    // #[test]
    // #[should_panic]
    // fn assign_negative_size() {
    //     let s = Size::new(-1, -1);
    // }

    #[test]
    #[should_panic]
    fn assign_over_8_size() {
        Size::new(100, 100);
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
    //     let c = Coords::new(-2, -3);
    // }

    #[test]
    #[should_panic]
    fn assign_over_7_coords() {
        Coords::new(8, 8);
    }

    #[test]
    fn assign_rel_coords() {
        let r = RelCoords::new(-1, 1);

        assert_eq!(-1, r.x);
        assert_eq!(1, r.y);
    }

    #[test]
    fn check_within_coords_true() {
        let c = Coords::new(0, 0);
        let r = RelCoords::new(1, 1);

        assert_eq!(Coords::check_within_coords(c, r), true);
    }

    #[test]
    fn check_within_coords_false_x_positive() {
        let c = Coords::new(7, 6);
        let r = RelCoords::new(1, 1);

        assert_eq!(Coords::check_within_coords(c, r), false);
    }

    #[test]
    fn check_within_coords_false_y_positive() {
        let c = Coords::new(6, 7);
        let r = RelCoords::new(1, 1);

        assert_eq!(Coords::check_within_coords(c, r), false);
    }

    #[test]
    fn check_within_coords_false_x_negative() {
        let c = Coords::new(0, 0);
        let r = RelCoords::new(-1, 0);

        assert_eq!(Coords::check_within_coords(c, r), false);
    }

    #[test]
    fn check_within_coords_false_y_negative() {
        let c = Coords::new(0, 0);
        let r = RelCoords::new(0, -1);

        assert_eq!(Coords::check_within_coords(c, r), false);
    }

    #[test]
    fn test_coords_and_rel_coords_result() {
        assert_eq!(Coords::new(3, 3), Coords::coords_and_rel_coords_result(
            Coords::new(5, 4), RelCoords::new(-2, -1))
        )
    }
}