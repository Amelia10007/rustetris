use super::*;

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

    /// このROIに含まれる格子点を列挙する．
    /// このメソッドで返される`iterator`は，まずROIの左上の座標を返し，順に右側の座標を返していく．
    /// 最上行の列挙が終わった後，続けて2行目の点を左端から右端へ順に列挙する．
    /// この操作を最下行まで繰り返す．
    pub fn iter_pos(&self) -> impl IntoIterator<Item = Pos> {
        let width = self.size.x().as_positive_index().unwrap();
        let height = self.size.y().as_positive_index().unwrap();
        let left_top = self.left_top;
        (0..height).map(|y| below(y as i8)).flat_map(move |y| {
            (0..width)
                .map(|x| right(x as i8))
                .map(move |x| left_top + x + y)
        })
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

    #[test]
    fn test_iter_pos() {
        let left_top = Pos(PosX::right(4), PosY::below(5));
        let size = right(2) + below(3);
        let roi = RegionOfInterest::new(left_top, size);
        let mut iter = roi.iter_pos().into_iter();

        assert_eq!(Some(Pos::origin() + right(4) + below(5)), iter.next());
        assert_eq!(Some(Pos::origin() + right(5) + below(5)), iter.next());
        assert_eq!(Some(Pos::origin() + right(4) + below(6)), iter.next());
        assert_eq!(Some(Pos::origin() + right(5) + below(6)), iter.next());
        assert_eq!(Some(Pos::origin() + right(4) + below(7)), iter.next());
        assert_eq!(Some(Pos::origin() + right(5) + below(7)), iter.next());
        assert!(iter.next().is_none());
    }
}
