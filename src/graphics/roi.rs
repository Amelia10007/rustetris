use crate::geometry::*;

/// 注目領域ROI(Region of interest)を表す．
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegionOfInterest {
    /// 親の座標系に対する，このROIの左上の点の座標．
    pub left_top: Pos,
    /// このROIのサイズ．
    pub size: Movement,
}

impl RegionOfInterest {
    /// # Panics
    /// `size`のxまたはy成分が負の場合．
    pub fn new(left_top: Pos, size: Movement) -> RegionOfInterest {
        debug_assert!(size.x().as_positive_index().is_some());
        debug_assert!(size.y().as_positive_index().is_some());
        Self { left_top, size }
    }

    /// このROIの右下の点の座標．
    pub fn right_below(&self) -> Pos {
        self.left_top + self.size + left(1) + above(1)
    }

    /// 指定した点がこのROIの内部に存在するか返す．
    pub fn contains(&self, pos: Pos) -> bool {
        let right_below = self.left_top + self.size;

        pos.x() >= self.left_top.x()
            && pos.x() < right_below.x()
            && pos.y() >= self.left_top.y()
            && pos.y() < right_below.y()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let left_top = Pos(PosX::right(2), PosY::below(3));
        let size = right(1) + below(1);
        let _roi = RegionOfInterest::new(left_top, size);
    }

    #[test]
    #[should_panic]
    fn test_new_negative_width() {
        let left_top = Pos(PosX::right(2), PosY::below(3));
        let size = left(1) + below(1);
        let _roi = RegionOfInterest::new(left_top, size);
    }

    #[test]
    #[should_panic]
    fn test_new_negative_height() {
        let left_top = Pos(PosX::right(2), PosY::below(3));
        let size = right(1) + above(1);
        let _roi = RegionOfInterest::new(left_top, size);
    }

    #[test]
    fn test_right_below() {
        let left_top = Pos(PosX::right(2), PosY::below(3));
        let size = right(5) + below(6);
        let roi = RegionOfInterest::new(left_top, size);
        assert_eq!(
            Pos::origin() + right(2 + 5 - 1) + below(3 + 6 - 1),
            roi.right_below()
        );
    }

    #[test]
    fn test_contains() {
        let left_top = Pos(PosX::right(2), PosY::below(3));
        let size = right(5) + below(6);
        let roi = RegionOfInterest::new(left_top, size);

        // ROIの4隅
        assert!(roi.contains(left_top));
        assert!(roi.contains(left_top + right(4)));
        assert!(roi.contains(left_top + below(5)));
        assert!(roi.contains(roi.right_below()));
        // ROIの外
        assert!(!roi.contains(left_top + left(1)));
        assert!(!roi.contains(left_top + above(1)));
        assert!(!roi.contains(roi.right_below() + right(1)));
        assert!(!roi.contains(roi.right_below() + below(1)));
    }
}
