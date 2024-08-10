use anchor_lang::error_code;

#[error_code]
pub enum TicTacToeError {
    InvalidTile,
    TileOutOfBounds,
    TileAlreadySet,
    GameAlreadyOver,
    NotPlayersTurn,
    GameAlreadyStarted,
}
