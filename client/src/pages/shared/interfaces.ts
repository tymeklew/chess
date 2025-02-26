import { Colour , PieceType} from "./enum";

interface Piece {
  colour: Colour;
  type : PieceType,
}

interface Move {
  from : [number , number];
  to : [number , number];
}


export type { Piece , Move }