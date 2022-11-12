#[derive(Clone, Copy)]
pub enum Piece {
    Knight,
    Queen,
}

pub enum KnightMove {
    NNE,
    ENE,
    ESE,
    SSE,
    SSW,
    WSW,
    WNW,
    NNW,
}

impl KnightMove {
    pub fn coord_delta(&self) -> (i8, i8) {
        match self {
            KnightMove::NNE => (1, -2),
            KnightMove::ENE => (2, -1),
            KnightMove::ESE => (2, 1),
            KnightMove::SSE => (1, 2),
            KnightMove::SSW => (-1, 2),
            KnightMove::WSW => (-2, 1),
            KnightMove::WNW => (-2, -1),
            KnightMove::NNW => (-1, -2),
        }
    }
}
