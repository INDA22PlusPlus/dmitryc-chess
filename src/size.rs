#[derive(Debug, PartialEq, Eq)]
pub struct Size {
    pub w:u8,
    pub h:u8,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assign_zeros() {
        let s = Size{w:0, h:0};
        assert_eq!(s.w, 0);
        assert_eq!(s.h, 0);
    }

    #[test]
    fn assign_ones() {
        let s = Size{w:1, h:1};
        assert_eq!(s.w, 1);
        assert_eq!(s.h, 1);
    }

    #[test]
    fn assign_different_values() {
        let s = Size{w:10, h:100};
        assert_eq!(s.w, 10);
        assert_eq!(s.h, 100);
    }

    #[test]
    fn assign_u8_limit_values() {
        let s = Size{w:255, h:255};
        assert_eq!(s.w, 255);
        assert_eq!(s.h, 255);
    }

    //TODO: How to write this type of test

    // #[test]
    // #[should_panic]
    // fn assign_larger_value_than_u8() {
    //     let s = Size{w:1000, h:1000};
    // }
}