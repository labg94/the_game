
#[derive(Debug, thiserror::Error)]
pub enum GameError{
    #[error("Invalid card play {0}")]
    InvalidCardPlay(String)
}