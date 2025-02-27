import { Move, Piece } from "../../pages/shared/interfaces";
import WhiteRook from "../../assets/white_rook.png";
import "./board.css";
import React, { useEffect } from "react";

interface BoardProps {
  board: (Piece | null)[][];
  moves : Move[];
  setActiveSquare : React.Dispatch<React.SetStateAction<[number, number] | null>>;
  activeSquare : [number , number] | null;
  sendMove : (move : Move) => void;
}

const ALPHABET = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
export default function Board(props : BoardProps) {
  //t Make text numbering and lettering inverted
  useEffect(() => {
  } , [props.activeSquare]);

  function handleTileClick( i : number , j : number , piece : Piece | null) {
    if (piece && piece.colour == "white") {
      props.setActiveSquare([i , j]);
    } else {
      if (props.activeSquare) {
        let move = props.moves.find(move => move.from[0] == props.activeSquare?.[1] && move.from[1] == props.activeSquare?.[0] && move.to[0] == j && move.to[1] == i);
        if (move) {
          // Send move to server
          console.log(move);
          props.setActiveSquare(null);
          props.sendMove(move);
        }
      }
      props.setActiveSquare(null);
    }
  }
  return (
    <div className="board">
      {props.board.map((row, i) => {
        return row.map((piece, j) => {
          let isActiveSquare = i == props.activeSquare?.[0] && j == props.activeSquare?.[1];
          return <div key={`${i}-${j}`} className={`tile tile-${(i + j) % 2 == 0 ? 'light' : 'dark'} ${isActiveSquare ? 'active' : ''} `} onClick={() => handleTileClick(i , j , piece)}>
            {j == 0 ? <p className="upper-number numbering">{ 8 - i}</p> : ""}
            {i == 7 ? <p className="lower-letter numbering">{ALPHABET[j]}</p> : ""}
            {piece == null ? "" : <img className="piece" src={`/assets/${piece.colour}_${piece.type}.png`} alt="white rook" />}
            { props.activeSquare && props.moves.some(move => move.to[0] == j && move.to[1] == i && move.from[1] == props.activeSquare?.[0] && move.from[0] == props.activeSquare?.[1] ) ? <div className="highlight"></div> : ""}
          </div>;
      });
    })}

    </div>
  );
}