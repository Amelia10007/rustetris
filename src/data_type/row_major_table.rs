use crate::data_type::{Table, TableIndex, TableMut, TableSize};
use std::fmt;
use std::ops::{Index, IndexMut};

/// 2次元方向に固定長の大きさをもつ，行優先テーブルを表す．
/// 実体は1次元のVec<T>なので，Vec<Vec<T>>よりも動作は高速．
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RowMajorTable<T> {
    /// このテーブルに配置されているデータ．
    ///
    /// Vecの要素はまず0行目0列目から右方向に配置されていると解釈され，
    /// その後1行目0列目から，2行目0列目から...と同様の配置が繰り返されていると解釈される．
    vec: Vec<T>,
    /// このテーブルのサイズ(width, height)．
    size: TableSize,
}

pub struct TableRowIter<'a, T> {
    table: &'a RowMajorTable<T>,
    current_row: usize,
}

pub struct TableRowIterMut<'a, T> {
    table: &'a mut RowMajorTable<T>,
    current_row: usize,
}

pub struct TableColumnIter<'a, T> {
    table: &'a RowMajorTable<T>,
    current_column: usize,
}

pub struct TableColumnIterItem<'a, T> {
    table: &'a RowMajorTable<T>,
    column: usize,
    current_row: usize,
}

impl<T> RowMajorTable<T> {
    /// 指定したVecとテーブルサイズからテーブルを作成する．
    ///
    /// Vecの要素はまず0行目0列目から右方向に配置され，
    /// その後1行目0列目から，2行目0列目から...と同様の配置が繰り返される．
    /// # Panics
    /// 1. Vecとサイズの要素数が一致しない場合．
    /// 1. テーブルの横方向，または縦方向のサイズに0が指定された場合．
    ///
    /// # Examples
    /// ```
    /// use data_structure::{Table, RowMajorTable, TableSize, TableIndex};
    ///
    /// let v = vec![100, 200, 300, 400];
    /// let size = TableSize::new(2, 2);
    /// let table = RowMajorTable::from_vec(v, size);
    /// assert_eq!(table.size(), size);
    /// assert_eq!(100, table[TableIndex::new(0, 0)]);
    /// assert_eq!(200, table[TableIndex::new(1, 0)]);
    /// assert_eq!(300, table[TableIndex::new(0, 1)]);
    /// assert_eq!(400, table[TableIndex::new(1, 1)]);
    /// ```
    pub fn from_vec(vec: Vec<T>, size: TableSize) -> RowMajorTable<T> {
        assert!(size.x > 0);
        assert!(size.y > 0);
        assert!(vec.len() == size.x * size.y);
        Self { vec, size }
    }

    /// 指定した2次元配列からテーブルを作成する．
    ///
    /// # Panics
    /// 2次元配列の各要素(Vec<T>)の要素数が一致しない場合．
    ///
    /// 2次元配列の要素数が0の場合．
    ///
    /// 2次元配列の要素に要素数0のVec<T>が存在する場合．
    /// # Examples
    /// ```
    /// use data_structure::{Table, RowMajorTable, TableSize, TableIndex};
    ///
    /// let table = RowMajorTable::from_lines(vec![vec![5, 6], vec![7, 8], vec![9, 10]]);
    /// assert_eq!(TableSize::new(2, 3), table.size());
    /// assert_eq!(5, table[TableIndex::new(0, 0)]);
    /// assert_eq!(6, table[TableIndex::new(1, 0)]);
    /// assert_eq!(7, table[TableIndex::new(0, 1)]);
    /// assert_eq!(8, table[TableIndex::new(1, 1)]);
    /// assert_eq!(9, table[TableIndex::new(0, 2)]);
    /// assert_eq!(10, table[TableIndex::new(1, 2)]);
    /// ```
    pub fn from_lines(lines: Vec<Vec<T>>) -> RowMajorTable<T> {
        assert!(lines.len() > 0);
        let width = lines[0].len();
        assert!(width > 0);
        for line in lines.iter() {
            assert_eq!(line.len(), width);
        }
        let height = lines.len();
        let mut v = vec![];
        for line in lines.into_iter() {
            v.extend(line);
        }
        Self {
            vec: v,
            size: TableSize::new(width, height),
        }
    }

    /// このテーブルの各要素を順に返すイテレータを生成する．
    pub fn iter_items(&self) -> impl Iterator<Item = &T> {
        self.vec.iter()
    }

    /// このテーブルの各行への参照を順に返すイテレータを生成する．
    /// # Examples
    /// ```
    /// use data_structure::RowMajorTable;
    ///
    /// // create 2-row 3-column table
    /// let table = RowMajorTable::from_lines(vec![vec![0, 1, 2], vec![3, 4, 5]]);
    /// let mut iter = table.iter_row();
    /// // first row
    /// assert_eq!(Some(vec![0, 1, 2].as_slice()), iter.next());
    /// // second, the last row
    /// assert_eq!(Some(vec![3, 4, 5].as_slice()), iter.next());
    /// // no more row
    /// assert_eq!(None, iter.next());
    /// ```
    pub fn iter_row(&self) -> TableRowIter<'_, T> {
        TableRowIter::new(self)
    }

    /// このテーブルの各行への可変参照を順に返すイテレータを生成する．
    /// # Examples
    /// ```
    /// use data_structure::{RowMajorTable, TableIndex};
    ///
    /// // create 2-row 3-column table
    /// let mut table = RowMajorTable::from_lines(vec![vec![0, 1], vec![2, 3]]);
    /// for (i, row) in table.iter_row_mut().enumerate() {
    ///     row[0] = 10 * (i + 1);
    /// }
    /// assert_eq!(10, table[TableIndex::new(0, 0)]);
    /// assert_eq!(1, table[TableIndex::new(1, 0)]);
    /// assert_eq!(20, table[TableIndex::new(0, 1)]);
    /// assert_eq!(3, table[TableIndex::new(1, 1)]);
    /// ```
    pub fn iter_row_mut(&mut self) -> TableRowIterMut<'_, T> {
        TableRowIterMut::new(self)
    }

    /// このテーブルの各列への参照を順に返すイテレータを生成する．
    /// # Examples
    /// ```
    /// use data_structure::RowMajorTable;
    ///
    /// // create 2-row 3-column table
    /// let table = RowMajorTable::from_lines(vec![vec![0, 1, 2], vec![3, 4, 5]]);
    /// let mut column_iter = table.iter_column();
    /// // first column
    /// assert_eq!(
    ///     vec![&0, &3],
    ///     column_iter.next().unwrap().collect::<Vec<_>>()
    /// );
    /// //second column
    /// assert_eq!(
    ///     vec![&1, &4],
    ///     column_iter.next().unwrap().collect::<Vec<_>>()
    /// );
    /// // third, the last column
    /// assert_eq!(
    ///     vec![&2, &5],
    ///     column_iter.next().unwrap().collect::<Vec<_>>()
    /// );
    /// // no more column
    /// assert!(column_iter.next().is_none());
    /// ```
    pub fn iter_column(&self) -> TableColumnIter<'_, T> {
        TableColumnIter::new(self)
    }

    /// このテーブルの情報を格納している1次元配列全体にアクセス可能なスライスを返す．
    pub fn as_raw_slice(&self) -> &[T] {
        &self.vec
    }

    /// このテーブルの情報を格納している1次元配列全体にアクセス可能なスライスを返す．
    pub fn as_raw_slice_mut(&mut self) -> &mut [T] {
        &mut self.vec
    }

    /// このテーブルの情報を格納している1次元配列を返す．
    pub fn into_inner(self) -> Vec<T> {
        self.vec
    }

    /// このテーブルの各要素に指定した関数を適用した結果を，このテーブルと同サイズのテーブルとして返す．
    pub fn map<U, F>(self, f: F) -> RowMajorTable<U>
    where
        F: Fn(T) -> U,
    {
        let vec = self.vec.into_iter().map(f).collect();
        RowMajorTable::from_vec(vec, self.size)
    }

    /// 指定した2次元インデックスに対応する，生配列へのインデックス`index`を返す．
    /// # Returns
    /// `Some(index)`．
    /// 引数`index`がこのテーブルの範囲外であった場合は`None`．
    pub fn raw_index_of(&self, index: TableIndex) -> Option<usize> {
        if index.x < self.width() && index.y < self.height() {
            Some(index.y * self.size.x + index.x)
        } else {
            None
        }
    }
}

impl<T: Clone> RowMajorTable<T> {
    /// 指定した値で初期化されたテーブルを返す．
    /// # Panics
    /// テーブルの横方向，または縦方向のサイズに0が指定された場合．
    pub fn from_fill(init: T, size: TableSize) -> RowMajorTable<T> {
        RowMajorTable::from_vec(vec![init; size.x * size.y], size)
    }
}

impl<T> Index<TableIndex> for RowMajorTable<T> {
    type Output = T;

    fn index(&self, index: TableIndex) -> &Self::Output {
        self.get(index).expect(&format!(
            "Out of table. Table size: {} index: {}",
            self.size(),
            index
        ))
    }
}

impl<T> IndexMut<TableIndex> for RowMajorTable<T> {
    fn index_mut(&mut self, index: TableIndex) -> &mut Self::Output {
        let size = self.size();
        self.get_mut(index).expect(&format!(
            "Out of table. Table size: {} index: {}",
            size, index
        ))
    }
}

impl<T> Table for RowMajorTable<T> {
    type Item = T;

    fn width(&self) -> usize {
        self.size().x
    }

    fn height(&self) -> usize {
        self.size().y
    }

    fn size(&self) -> TableSize {
        self.size
    }

    fn get(&self, index: TableIndex) -> Option<&Self::Item> {
        self.raw_index_of(index).and_then(|i| self.vec.get(i))
    }
}

impl<T> TableMut for RowMajorTable<T> {
    fn get_mut(&mut self, index: TableIndex) -> Option<&mut Self::Item> {
        self.raw_index_of(index)
            .and_then(move |i| self.vec.get_mut(i))
    }
}

impl<T: fmt::Display> fmt::Display for RowMajorTable<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.iter_row() {
            for item in row.iter() {
                write!(f, "{} ", item)?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

impl<'a, T> TableRowIter<'a, T> {
    const fn new(table: &'a RowMajorTable<T>) -> TableRowIter<'a, T> {
        Self {
            table,
            current_row: 0,
        }
    }
}

impl<'a, T> Iterator for TableRowIter<'a, T> {
    type Item = &'a [T];
    fn next(&mut self) -> Option<Self::Item> {
        let start_index = self.table.width() * self.current_row;
        let end_index = start_index + self.table.width();

        self.current_row += 1;

        if end_index <= self.table.vec.len() {
            Some(&self.table.vec[start_index..end_index])
        } else {
            None
        }
    }
}

impl<'a, T> TableRowIterMut<'a, T> {
    fn new(table: &'a mut RowMajorTable<T>) -> TableRowIterMut<'a, T> {
        Self {
            table,
            current_row: 0,
        }
    }
}

impl<'a, T> Iterator for TableRowIterMut<'a, T> {
    type Item = &'a mut [T];
    fn next(&mut self) -> Option<Self::Item> {
        let start_index = self.table.width() * self.current_row;
        let end_index = start_index + self.table.width();

        self.current_row += 1;

        self.table.vec.iter_mut();

        if end_index <= self.table.vec.len() {
            // unsafeブロックを利用しないと，所有権ルールを抜け出せない．
            let slice = unsafe {
                let ptr = self.table.vec.as_mut_ptr().add(start_index);
                std::slice::from_raw_parts_mut(ptr, self.table.width())
            };
            Some(slice)
        } else {
            None
        }
    }
}

impl<'a, T> TableColumnIter<'a, T> {
    const fn new(table: &'a RowMajorTable<T>) -> TableColumnIter<'a, T> {
        Self {
            table,
            current_column: 0,
        }
    }
}

impl<'a, T> Iterator for TableColumnIter<'a, T> {
    type Item = TableColumnIterItem<'a, T>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_column >= self.table.width() {
            None
        } else {
            let iter_item = TableColumnIterItem::new(self.table, self.current_column);
            self.current_column += 1;
            Some(iter_item)
        }
    }
}

impl<'a, T> TableColumnIterItem<'a, T> {
    const fn new(table: &'a RowMajorTable<T>, column: usize) -> TableColumnIterItem<'a, T> {
        Self {
            table,
            column,
            current_row: 0,
        }
    }
}

impl<'a, T> Iterator for TableColumnIterItem<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        let value = self
            .table
            .get(TableIndex::new(self.column, self.current_row));
        self.current_row += 1;
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_vec() {
        let v = vec![0, 1, 2, 3];
        let size = TableSize::new(2, 2);
        let table = RowMajorTable::from_vec(v, size);
        assert_eq!(table.size(), size);
    }

    #[test]
    #[should_panic]
    fn test_from_vec_none() {
        let v = vec![0, 1, 2, 3];
        let size = TableSize::new(2, 1);
        let _table = RowMajorTable::from_vec(v, size);
    }

    #[test]
    #[should_panic]
    fn test_from_vec_zero_none() {
        let v = Vec::<i32>::new();
        let size = TableSize::new(0, 1);
        let _table = RowMajorTable::from_vec(v, size);
    }

    #[test]
    fn test_from_fill() {
        let x = 1;
        let size = TableSize::new(2, 2);
        let table = RowMajorTable::from_fill(x, size);
        assert_eq!(table.size(), size);
    }

    #[test]
    #[should_panic]
    fn test_from_fill_none_width() {
        let x = 1;
        let size = TableSize::new(0, 2);
        let _table = RowMajorTable::from_fill(x, size);
    }

    #[should_panic]
    #[test]
    fn test_from_fill_none_height() {
        let x = 1;
        let size = TableSize::new(2, 0);
        let _table = RowMajorTable::from_fill(x, size);
    }

    #[test]
    fn test_from_lines() {
        let table = RowMajorTable::from_lines(vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]]);
        assert_eq!(TableSize::new(3, 3), table.size());
        assert_eq!(0, table[TableIndex::new(0, 0)]);
        assert_eq!(1, table[TableIndex::new(1, 0)]);
        assert_eq!(2, table[TableIndex::new(2, 0)]);
        assert_eq!(3, table[TableIndex::new(0, 1)]);
        assert_eq!(4, table[TableIndex::new(1, 1)]);
        assert_eq!(5, table[TableIndex::new(2, 1)]);
        assert_eq!(6, table[TableIndex::new(0, 2)]);
        assert_eq!(7, table[TableIndex::new(1, 2)]);
        assert_eq!(8, table[TableIndex::new(2, 2)]);
    }

    #[test]
    #[should_panic]
    fn test_from_lines_none_empty() {
        let _table = RowMajorTable::from_lines(Vec::<Vec<i32>>::new());
    }

    #[test]
    #[should_panic]
    fn test_from_lines_none_jagged() {
        let jagged_lines = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7]];
        let _table = RowMajorTable::from_lines(jagged_lines);
    }

    #[test]
    fn test_get() {
        let table = RowMajorTable::from_lines(vec![vec![0, 1], vec![2, 3]]);
        assert_eq!(Some(&0), table.get(TableIndex::new(0, 0)));
        assert_eq!(Some(&1), table.get(TableIndex::new(1, 0)));
        assert_eq!(Some(&2), table.get(TableIndex::new(0, 1)));
        assert_eq!(Some(&3), table.get(TableIndex::new(1, 1)));

        // None due to index out of range
        assert!(table.get(TableIndex::new(0, 2)).is_none());
        assert!(table.get(TableIndex::new(2, 0)).is_none());
        assert!(table.get(TableIndex::new(2, 2)).is_none());
    }

    #[test]
    fn test_get_mut() {
        let mut table = RowMajorTable::from_lines(vec![vec![0, 1], vec![2, 3]]);
        assert_eq!(Some(&mut 0), table.get_mut(TableIndex::new(0, 0)));
        assert_eq!(Some(&mut 1), table.get_mut(TableIndex::new(1, 0)));
        assert_eq!(Some(&mut 2), table.get_mut(TableIndex::new(0, 1)));
        assert_eq!(Some(&mut 3), table.get_mut(TableIndex::new(1, 1)));

        // overwrite
        *table.get_mut(TableIndex::new(1, 0)).unwrap() = 100;

        assert_eq!(Some(&mut 0), table.get_mut(TableIndex::new(0, 0)));
        assert_eq!(Some(&mut 100), table.get_mut(TableIndex::new(1, 0)));
        assert_eq!(Some(&mut 2), table.get_mut(TableIndex::new(0, 1)));
        assert_eq!(Some(&mut 3), table.get_mut(TableIndex::new(1, 1)));

        // None due to index out of range
        assert!(table.get_mut(TableIndex::new(0, 2)).is_none());
        assert!(table.get_mut(TableIndex::new(2, 0)).is_none());
        assert!(table.get_mut(TableIndex::new(2, 2)).is_none());
    }

    #[test]
    fn test_index() {
        let table = RowMajorTable::from_lines(vec![vec![0, 1], vec![2, 3]]);
        assert_eq!(0, table[TableIndex::new(0, 0)]);
        assert_eq!(1, table[TableIndex::new(1, 0)]);
        assert_eq!(2, table[TableIndex::new(0, 1)]);
        assert_eq!(3, table[TableIndex::new(1, 1)]);
    }

    #[test]
    #[should_panic]
    fn test_index_out_of_range_x() {
        let table = RowMajorTable::from_lines(vec![vec![0, 1], vec![2, 3]]);
        table[TableIndex::new(2, 0)];
    }

    #[test]
    #[should_panic]
    fn test_index_out_of_range_y() {
        let table = RowMajorTable::from_lines(vec![vec![0, 1], vec![2, 3]]);
        table[TableIndex::new(0, 2)];
    }

    #[test]
    fn test_index_mut() {
        let mut table = RowMajorTable::from_lines(vec![vec![0, 1], vec![2, 3]]);
        table[TableIndex::new(0, 0)] = 4;
        table[TableIndex::new(1, 0)] = 5;
        table[TableIndex::new(0, 1)] = 6;
        table[TableIndex::new(1, 1)] = 7;
        assert_eq!(4, table[TableIndex::new(0, 0)]);
        assert_eq!(5, table[TableIndex::new(1, 0)]);
        assert_eq!(6, table[TableIndex::new(0, 1)]);
        assert_eq!(7, table[TableIndex::new(1, 1)]);
    }

    #[test]
    #[should_panic]
    fn test_index_mut_out_of_range_x() {
        let mut table = RowMajorTable::from_lines(vec![vec![0, 1], vec![2, 3]]);
        table[TableIndex::new(2, 0)] = 10;
    }

    #[test]
    #[should_panic]
    fn test_index_mut_out_of_range_y() {
        let mut table = RowMajorTable::from_lines(vec![vec![0, 1], vec![2, 3]]);
        table[TableIndex::new(0, 2)] = 10;
    }

    #[test]
    fn test_size() {
        let size = TableSize::new(2, 4);
        let table = RowMajorTable::from_fill(1, size);
        assert_eq!(table.size(), size);
        assert_eq!(2, table.width());
        assert_eq!(4, table.height());
    }

    #[test]
    fn test_iter_items() {
        let table = RowMajorTable::from_lines(vec![vec![0, 1], vec![2, 3]]);
        let mut iterator = table.iter_items();
        assert_eq!(Some(&0), iterator.next());
        assert_eq!(Some(&1), iterator.next());
        assert_eq!(Some(&2), iterator.next());
        assert_eq!(Some(&3), iterator.next());
        assert_eq!(None, iterator.next());
    }

    #[test]
    fn test_iter_row() {
        // create 2-row 3-column table
        let table = RowMajorTable::from_lines(vec![vec![0, 1, 2], vec![3, 4, 5]]);
        let mut iter = table.iter_row();
        assert_eq!(Some(vec![0, 1, 2].as_slice()), iter.next());
        assert_eq!(Some(vec![3, 4, 5].as_slice()), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn test_iter_row_mut() {
        let mut table = RowMajorTable::from_lines(vec![vec![0, 1], vec![2, 3]]);
        for (i, row) in table.iter_row_mut().enumerate() {
            row[0] = 10 * (i + 1);
        }
        assert_eq!(10, table[TableIndex::new(0, 0)]);
        assert_eq!(1, table[TableIndex::new(1, 0)]);
        assert_eq!(20, table[TableIndex::new(0, 1)]);
        assert_eq!(3, table[TableIndex::new(1, 1)]);
    }

    #[test]
    fn test_iter_column() {
        // create 2-row 3-column table
        let table = RowMajorTable::from_lines(vec![vec![0, 1, 2], vec![3, 4, 5]]);
        let mut column_iter = table.iter_column();
        // first column
        assert_eq!(
            vec![&0, &3],
            column_iter.next().unwrap().collect::<Vec<_>>()
        );
        //second column
        assert_eq!(
            vec![&1, &4],
            column_iter.next().unwrap().collect::<Vec<_>>()
        );
        // third, the last column
        assert_eq!(
            vec![&2, &5],
            column_iter.next().unwrap().collect::<Vec<_>>()
        );
        // no more column
        assert!(column_iter.next().is_none());
    }

    #[test]
    fn test_as_raw_slice() {
        let table = RowMajorTable::from_lines(vec![vec![0, 1, 2], vec![3, 4, 5]]);
        assert_eq!(&[0, 1, 2, 3, 4, 5], table.as_raw_slice());
    }

    #[test]
    fn test_as_raw_slice_mut() {
        let mut table = RowMajorTable::from_lines(vec![vec![0, 1, 2], vec![3, 4, 5]]);

        let slice = table.as_raw_slice_mut();
        assert_eq!(&[0, 1, 2, 3, 4, 5], slice);

        slice[1] = 100;
        assert_eq!(&[0, 100, 2, 3, 4, 5], slice);
    }

    #[test]
    fn test_as_raw_vec() {
        let table = RowMajorTable::from_lines(vec![vec![0, 1, 2], vec![3, 4, 5]]);
        assert_eq!(vec![0, 1, 2, 3, 4, 5], table.into_inner());
    }

    #[test]
    fn test_map() {
        let table = RowMajorTable::from_lines(vec![vec![0, 1], vec![2, 3]]);
        let mapped = table.map(|x| x * x);
        assert_eq!(0, mapped[TableIndex::new(0, 0)]);
        assert_eq!(1, mapped[TableIndex::new(1, 0)]);
        assert_eq!(4, mapped[TableIndex::new(0, 1)]);
        assert_eq!(9, mapped[TableIndex::new(1, 1)]);
    }

    #[test]
    fn test_display() {
        use std::fmt::Write;

        let table = RowMajorTable::from_lines(vec![vec![0, 1], vec![2, 3]]);
        let mut s = String::new();
        write!(s, "{}", table).unwrap();
        let mut lines = s.lines();
        assert_eq!(Some("0 1 "), lines.next());
        assert_eq!(Some("2 3 "), lines.next());
        assert_eq!(None, lines.next());
    }
}
