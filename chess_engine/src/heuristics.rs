pub const PAWN_TABLE: [[i32; 8]; 8] = [
    [0, 0, 0, 0, 0, 0, 0, 0],
    [50, 50, 50, 50, 50, 50, 50, 50], // Encourage central pawns
    [10, 10, 20, 30, 30, 20, 10, 10], // Support center control
    [5, 5, 10, 25, 25, 10, 5, 5],
    [0, 0, 0, 20, 20, 0, 0, 0],       // Discourage early pushes
    [5, -10, -20, 0, 0, -20, -10, 5], // Avoid weak moves
    [5, 10, 10, -20, -20, 10, 10, 5],
    [0, 0, 0, 0, 0, 0, 0, 0], // Pawns should not be overextended early
];
