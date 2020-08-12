pub use console::Key;

/// メニュー画面で使用可能な操作を表す．
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuCommand {
    /// ひとつ上の項目を選択する．
    Up,
    /// ひとつ下の項目を選択する．
    Down,
    /// 現在選択中の項目で選択を終了する．
    Proceed,
    /// ひとつ上層の選択画面へ戻る．
    Back,
}

/// ゲームプレイ画面で使用可能な操作を表す．
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameCommand {
    /// ブロックを1セルぶん左に移動させる．
    Left,
    /// ブロックを1セルぶん右に移動させる．
    Right,
    /// ブロックを1セルぶん下に移動させる．
    Down,
    /// ブロックを可能な限り下に移動させる．
    Drop,
    /// ブロックを時計回りに90度回転させる．
    RotateClockwise,
    /// ブロックを反時計回りに90度回転させる．
    RotateUnticlockwise,
    /// ホールド操作．
    /// ホールドブロックが存在する場合は，現在操作中のブロックとホールドブロックを交換する．
    /// ホールドブロックが存在しない場合は，現在操作中のブロックをホールドブロックとして，Nextブロック列の先頭ブロックを
    /// 新たな操作ブロックとする．
    Hold,
}

pub struct MenuInputMapper;

impl MenuInputMapper {
    pub fn map(&self, key: Key) -> Option<MenuCommand> {
        use Key::*;
        use MenuCommand::*;

        match key {
            Char('z') => Some(Proceed),
            Char('x') => Some(Back),
            ArrowUp => Some(Up),
            ArrowDown => Some(Down),
            _ => None,
        }
    }
}

pub struct SinglePlayerInputMapper;

impl SinglePlayerInputMapper {
    pub fn map(&self, key: Key) -> Option<GameCommand> {
        use GameCommand::*;
        use Key::*;

        match key {
            Char('z') => Some(RotateUnticlockwise),
            Char('x') => Some(RotateClockwise),
            Char('c') => Some(Hold),
            ArrowLeft => Some(Left),
            ArrowRight => Some(Right),
            ArrowUp => Some(Drop),
            ArrowDown => Some(Down),
            _ => None,
        }
    }
}

pub struct DoublePlayerInputMapper;

impl DoublePlayerInputMapper {
    pub fn map(&self, key: Key) -> (Option<GameCommand>, Option<GameCommand>) {
        unimplemented!()
    }
}
