/// セルを表す．
/// セルは，ブロックを構成する最小単位である．
/// また，フィールドに二次元格子状に配置されるものでもある．
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cell {
    /// 空セル．
    Empty,
    /// 通常のセル．
    Normal,
    /// ボムセル．
    Bomb,
    /// デカボムの左上を表すセル．
    BigBombUpperLeft,
    /// デカボムの左上以外に割り当てられるセル．
    BigBombPart,
}

impl Cell {
    /// このセルが空セルであるか返す．
    pub fn is_empty(&self) -> bool {
        match self {
            Cell::Empty => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Cell;

    #[test]
    fn test_is_empty() {
        assert!(Cell::Empty.is_empty());
        assert!(!Cell::Normal.is_empty());
        assert!(!Cell::Bomb.is_empty());
        assert!(!Cell::BigBombUpperLeft.is_empty());
        assert!(!Cell::BigBombPart.is_empty());
    }
}
