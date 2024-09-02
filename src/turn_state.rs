#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TurnState {
    AwaititngInput,
    PlayerTurn,
    MonsterTurn,
}