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
pub enum SingleGameCommand {
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
    /// 一時停止．
    Pause,
}

/// ユーザからの入力をシステム内で利用する操作に変換するトレイト．
pub trait InputMapper {
    fn map_to_menu_command(&self, key: Key) -> Option<MenuCommand>;

    fn map_to_single_game_command(&self, key: Key) -> Option<SingleGameCommand>;
}

pub struct SinglePlayerInputMapper;

impl InputMapper for SinglePlayerInputMapper {
    fn map_to_menu_command(&self, key: Key) -> Option<MenuCommand> {
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

    fn map_to_single_game_command(&self, key: Key) -> Option<SingleGameCommand> {
        use Key::*;
        use SingleGameCommand::*;

        match key {
            Char('z') => Some(RotateUnticlockwise),
            Char('x') => Some(RotateClockwise),
            Char('c') => Some(Hold),
            ArrowLeft => Some(Left),
            ArrowRight => Some(Right),
            ArrowUp => Some(Drop),
            ArrowDown => Some(Down),
            Home => Some(Pause),
            _ => None,
        }
    }
}
