import { ReactNode, useEffect, useState } from "react";
import { Board, ChatBox } from "../../components";
import { GiCrossedPistols, GiFlyingFlag, GiChessKing , GiRobotAntennas , GiPerson, GiPlayButton, GiChessQueen } from "react-icons/gi";
import { fromUCI, Move, Piece, toUCI } from "../shared/interfaces";
import "./game.css";
import { Colour, PieceType } from "../shared/enum";
import DEFAULT_BOARD from "../shared/const";

enum Status {
  Disconnected,
  Connecting,
  Connected,
}

const ALPHABET = "abcdefgh";
export default function Game() {
  const [messages, setMessages] = useState<string[]>([]);
  const [webSock, setWebSock] = useState<WebSocket>();
  const [status, setStatus] = useState<Status>(Status.Disconnected);
  const [board , setBoard] = useState<(Piece | null)[][]>([]);
  const [activeSquare , setActiveSquare] = useState<[number , number] | null>(null);
  const [legalMoves , setLegalMoves] = useState<Move[]>([]);
  const [promotionPiece , setPromotionPiece] = useState<PieceType | null>(null);

  useEffect(() => {
    console.log("Setting board");
    setBoard(DEFAULT_BOARD)
  } , []);

  useEffect(() => {
    console.log("MODIFIED BOARD");
  } , [board])

  function handleButtonClick(difficulty : string | null) {
    setStatus(Status.Connecting);
    let url;
    if (difficulty) {
      url = `ws://localhost:5173/api/ws/play?game=bot&side=white&difficulty=${difficulty}`
    } else {
      url = "ws://localhost:5173/api/ws/play?game=real?side=white"
    }
    const socket = new WebSocket(url);
    setWebSock(socket);
    // Connection opened
    socket.onopen = () => {
      setBoard(DEFAULT_BOARD);
      setStatus(Status.Connected);

      socket.send(JSON.stringify({type : "legal_moves"}))
    };
    socket.onclose = () => setStatus(Status.Disconnected);

    socket.onmessage = handleMessage;
  }

  async function sendMove(mv : Move) {
    console.log("Sending move: " + toUCI(mv));
    const temp = structuredClone(board);
    const fromPiece = temp[mv.from[1]][mv.from[0]];
    if (!fromPiece) {
      console.log("No piece at source position, cannot send move");
      return;
    }
    temp[mv.to[1]][mv.to[0]] = fromPiece;
    temp[mv.from[1]][mv.from[0]] = null;
    if (mv.promotion) {
      temp[mv.to[1]][mv.to[0]] = {
        colour: fromPiece.colour,
        type: mv.promotion
      };
    }
    webSock?.send(JSON.stringify({type: "move", data: toUCI(mv)}));
    webSock?.send(JSON.stringify({type: "legal_moves"}));
    setBoard(temp);
  }

  function handleMessage(evt : any) { 
    let msg = JSON.parse(evt.data);
    console.log("Handling message : " + msg.type + " : ");
    console.log(board);
    switch (msg.type) {
      case "move":
        let move = fromUCI(msg.data);
        setBoard(prevBoard => {
          const temp = structuredClone(prevBoard);
          const fromPiece = temp[move.from[1]][move.from[0]];
          if (!fromPiece) {
            console.log("No piece at source position, skipping move");
            return prevBoard;
          }
          temp[move.to[1]][move.to[0]] = fromPiece;
          temp[move.from[1]][move.from[0]] = null;
          if (move.promotion) {
            temp[move.to[1]][move.to[0]] = {
              colour: fromPiece.colour,
              type: move.promotion
            };
          }
          return temp;
        });
        break;
      case "legal_moves":
        console.log("Recieved legal moves");
        let mvs = msg.data.split(",").filter((m: string) => m !== "").map((move : string) => fromUCI(move)); 
        setLegalMoves(mvs);
        break;
      case "game_over":
        alert("Game Over : " + msg.data);
        break;
    }
  }


  function getStatus(): ReactNode {
    switch (status) {
      case Status.Connected:
        return <h2 className="status connected">Connected</h2>;
      case Status.Connecting:
        return <h2 className="status connecting">Connecting</h2>;
      case Status.Disconnected:
        return <h2 className="status disconnected">Disconnected</h2>;
    }
  }

  return (
    <div className="game-container">
      <div className="board-container">
        <Board board={board} moves={legalMoves} setActiveSquare={setActiveSquare} activeSquare={activeSquare} sendMove={sendMove} setPromotionPiece={setPromotionPiece}/>
        <div className="info-container">
          {getStatus()}
          <h2> Play real </h2>
          <button className="play-button" onClick={() => handleButtonClick(null)}> 
           <GiChessKing/> Play 
          </button>
          <h2>Play bot</h2>
          <button onClick={() => handleButtonClick("easy")}>
            <GiRobotAntennas/> Play Easy
          </button>
          <button onClick={() => handleButtonClick("medium")}>
            <GiRobotAntennas/> Play Medium 
          </button>
          <button onClick={() => handleButtonClick("hard")}>
            <GiRobotAntennas/> Play Hard 
          </button>
        </div>
      </div>
    </div>
  );
}
