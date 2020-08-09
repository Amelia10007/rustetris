use crate::geometry::*;
use take_if::TakeIf;

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

    /// 指定した点がこのROIの内部に存在するか返す．
    pub fn contains(&self, pos: Pos) -> bool {
        let maybe_x = pos
            .x()
            .as_positive_index()
            .take_if(|&x| x < self.size.x().as_positive_index().unwrap());
        let maybe_y = pos
            .y()
            .as_positive_index()
            .take_if(|&y| y < self.size.y().as_positive_index().unwrap());

        maybe_x.and(maybe_y).is_some()
    }
}
