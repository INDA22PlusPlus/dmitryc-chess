#[derive(Debug, PartialEq, Eq)]
pub struct Size {
    pub w:u8,
    pub h:u8,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Coords {
    pub x:u8,
    pub y:u8,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assign_zeros_size() {
        let s = Size{w:0, h:0};
        assert_eq!(s.w, 0);
        assert_eq!(s.h, 0);
    }

    #[test]
    fn assign_ones_size() {
        let s = Size{w:1, h:1};
        assert_eq!(s.w, 1);
        assert_eq!(s.h, 1);
    }

    #[test]
    fn assign_different_values_size() {
        let s = Size{w:10, h:100};
        assert_eq!(s.w, 10);
        assert_eq!(s.h, 100);
    }

    #[test]
    fn assign_u8_limit_values_size() {
        let s = Size{w:255, h:255};
        assert_eq!(s.w, 255);
        assert_eq!(s.h, 255);
    }

    //TODO: How to write this type of test

    // #[test]
    // #[should_panic]
    // fn assign_larger_value_than_u8_size() {
    //     let s = Size{w:1000, h:1000};
    // }

    #[test]
    fn assign_zeros_coords() {
        let s = Size{w:0, h:0};
        assert_eq!(s.w, 0);
        assert_eq!(s.h, 0);
    }

    #[test]
    fn assign_ones_coords() {
        let s = Size{w:1, h:1};
        assert_eq!(s.w, 1);
        assert_eq!(s.h, 1);
    }

    #[test]
    fn assign_different_values_coords() {
        let s = Size{w:10, h:100};
        assert_eq!(s.w, 10);
        assert_eq!(s.h, 100);
    }

    #[test]
    fn assign_u8_limit_values_coords() {
        let s = Size{w:255, h:255};
        assert_eq!(s.w, 255);
        assert_eq!(s.h, 255);
    }

    //TODO: How to write this type of test

    // #[test]
    // #[should_panic]
    // fn assign_larger_value_than_u8_coords() {
    //     let s = Size{w:1000, h:1000};
    // }
}