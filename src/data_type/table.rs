use crate::data_type::Pair;

/// テーブルのサイズ．
pub type TableSize = Pair<usize>;
/// テーブル内の特定の要素を指すためのインデックス．
pub type TableIndex = Pair<usize>;

/// 2次元方向に長方形のサイズを持つデータ構造を表す．
pub trait Table {
    type Item;

    /// このテーブルの幅を返す．
    fn width(&self) -> usize;

    /// このテーブルの高さを返す．
    fn height(&self) -> usize;

    /// このテーブルのサイズを返す．
    fn size(&self) -> TableSize;

    /// 指定した位置の要素への参照を返す．
    /// # Params
    /// 1. `index` テーブルの要素の位置．
    /// 有効な範囲は`TableIndex::new(0, 0)`から`TableIndex::new(width - 1, height - 1)`まで．
    ///
    /// # Returns
    /// 指定した位置の要素への参照を`Some(ref)`として返す．
    /// 指定位置がテーブルの範囲外だった場合は`None`を返す．
    fn get(&self, index: TableIndex) -> Option<&Self::Item>;
}

/// 2次元方向に長方形のサイズを持ち，各要素を書き換えることができるデータ構造を表す．
pub trait TableMut: Table {
    /// 指定した位置の要素への可変参照を返す．
    /// # Params
    /// 1. `index` テーブルの要素の位置．
    /// 有効な範囲は`TableIndex::new(0, 0)`から`TableIndex::new(width - 1, height - 1)`まで．
    ///
    /// # Returns
    /// 指定した位置の要素への参照を`Some(ref)`として返す．
    /// 指定位置がテーブルの範囲外だった場合は`None`を返す．
    fn get_mut(&mut self, index: TableIndex) -> Option<&mut Self::Item>;
}
