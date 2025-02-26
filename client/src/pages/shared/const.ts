import { Colour, PieceType } from "./enum";
import { Piece } from "./interfaces";

const DEFAULT_BOARD : (Piece | null)[][] = [
    [ { colour : Colour.Black , type : PieceType.Rook},
      {colour : Colour.Black , type : PieceType.Knight},
        {colour : Colour.Black , type : PieceType.Bishop},
        {colour : Colour.Black , type : PieceType.Queen},
        {colour : Colour.Black , type : PieceType.King},
        {colour : Colour.Black , type : PieceType.Bishop},
        {colour : Colour.Black , type : PieceType.Knight},
        {colour : Colour.Black , type : PieceType.Rook}
    ],
    [
        {colour : Colour.Black , type : PieceType.Pawn},
        {colour : Colour.Black , type : PieceType.Pawn},
        {colour : Colour.Black , type : PieceType.Pawn},
        {colour : Colour.Black , type : PieceType.Pawn},
        {colour : Colour.Black , type : PieceType.Pawn},
        {colour : Colour.Black , type : PieceType.Pawn},
        {colour : Colour.Black , type : PieceType.Pawn},
        {colour : Colour.Black , type : PieceType.Pawn}
    ],
    [null , null , null , null , null , null , null , null],
    [null , null , null , null , null , null , null , null],
    [null , null , null , null , null , null , null , null],
    [null , null , null , null , null , null , null , null],
    [
          {colour : Colour.White , type : PieceType.Pawn},
          {colour : Colour.White , type : PieceType.Pawn},
          {colour : Colour.White , type : PieceType.Pawn},
          {colour : Colour.White , type : PieceType.Pawn},
          {colour : Colour.White , type : PieceType.Pawn},
          {colour : Colour.White , type : PieceType.Pawn},
          {colour : Colour.White , type : PieceType.Pawn},
          {colour : Colour.White , type : PieceType.Pawn}
      ],
    [ { colour : Colour.White , type : PieceType.Rook},
        {colour : Colour.White , type : PieceType.Knight},
          {colour : Colour.White , type : PieceType.Bishop},
          {colour : Colour.White , type : PieceType.Queen},
          {colour : Colour.White , type : PieceType.King},
          {colour : Colour.White , type : PieceType.Bishop},
          {colour : Colour.White , type : PieceType.Knight},
          {colour : Colour.White , type : PieceType.Rook}
      ],
      
];

export default DEFAULT_BOARD;