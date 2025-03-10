import { useState } from "react";
import { fromUCI, Move , Piece, Square } from "./interfaces";

const real = useState;

class Game {
    legal_moves : Move[];
    pieces : Piece[];
    constructor() {
       this.legal_moves = [] 
       this.pieces = [];
    }

    // Handle websocket messages
    handleMessage(evt : any) {}

    // Convert from fen
    fromFen(fen : string) {
        // Only care about the board
        let board = fen.split(" ")[0];
    }
    // Convert to uci

    // Send move to server
    sendMove(mv : Move) {}


    // Handle tile click
    handleTileClick(sqr : Square) {}

}

