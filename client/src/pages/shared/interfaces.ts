import { Colour , PieceType} from "./enum";

interface Piece {
  colour: Colour;
  type : PieceType,
}

interface Move {
  from : [number , number];
  to : [number , number];
  promotion : PieceType | null;
}

const ALPHABET = "abcdefgh";
function toUCI(move : Move) {
  let from = `${ALPHABET[move.from[0]]}${8 - move.from[1]}`;
  let to = `${ALPHABET[move.to[0]]}${8 - move.to[1]}`;
  return `${from}${to}${move.promotion ? move.promotion[0] : ""}`;
}

function fromUCI(uci : string) : Move {
  let from : [number , number]  = [ALPHABET.indexOf(uci[0]) , 8 - parseInt(uci[1])];
  let to : [number , number] = [ALPHABET.indexOf(uci[2]) , 8 - parseInt(uci[3])];

  let promotion = null;

  if (uci.length == 5) {
      switch (uci[4]) {
          case "q":
              promotion = PieceType.Queen;
              break;
          case "r":
              promotion = PieceType.Rook;
              break;
          case "b":
              promotion = PieceType.Bishop;
              break;
          case "n":
              promotion = PieceType.Knight;
              break;
          default:
              break;
      }
  }

  return {
    from : from,
    to : to ,
    promotion : promotion 
  }
}

export type { Piece , Move };
export { fromUCI , toUCI };