use crate::{board::BitBoard, bootstrap::ROOK_ATTACKS, game::Square};

#[inline]
fn rook_attacks(sq : Square , occupied : BitBoard) -> BitBoard {
    let attacks = ROOK_ATTACKS[sq.to_idx()];
}