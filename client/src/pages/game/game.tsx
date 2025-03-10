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
    setBoard(DEFAULT_BOARD)
  } , []);

  function handleButtonClick() {
    setStatus(Status.Connecting);
    const socket = new WebSocket("ws://localhost:5173/api/ws/play?game=bot?side=white?difficulty=3");
    setWebSock(socket);
    // Connection opened
    socket.onopen = () => {
      setBoard(DEFAULT_BOARD);
      setStatus(Status.Connected);

      socket.send(JSON.stringify({type : "legal"}))
    };
    socket.onclose = () => setStatus(Status.Disconnected);

    socket.onmessage = handleMessage;
  }

  function sendMove(mv : Move) {
    let temp = board;
    temp[mv.to[1]][mv.to[0]] = temp[mv.from[1]][mv.from[0]];
    temp[mv.from[1]][mv.from[0]] = null;
    if (mv.promotion) {
      temp[mv.to[1]][mv.to[0]] = {
        colour : temp[mv.to[1]][mv.to[0]]?.colour || Colour.White,
        type : mv.promotion
      }
    }
    setBoard(temp);
    webSock?.send(JSON.stringify({type : "move" , data : toUCI(mv)}))
    webSock?.send(JSON.stringify({type : "legal"}))
  }

  function handleMessage(evt : any) { 
    let msg = JSON.parse(evt.data);
    switch (msg.type) {
      case "move":
        let move = fromUCI(msg.data);
        let temp = board;
        temp[move.to[1]][move.to[0]] = temp[move.from[1]][move.from[0]];
        temp[move.from[1]][move.from[0]] = null;
        if (move.promotion) {
          temp[move.to[1]][move.to[0]] = {
            colour : temp[move.to[1]][move.to[0]]?.colour || Colour.White,
            type : move.promotion
          }
        }
        setBoard(temp);
        break;
      case "legal_moves":
        let mvs = msg.data.split(",").map((move : string) => fromUCI(move)); 
        console.log(mvs);
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
          <button className="play-button"> 
           <GiChessKing/> Play 
          </button>
          <h2>Play bot</h2>
          <button>
            <GiRobotAntennas/> Play Easy
          </button>
          <button>
            <GiRobotAntennas/> Play Medium 
          </button>
          <button>
            <GiRobotAntennas/> Play Hard 
          </button>
        </div>
      </div>
    </div>
  );
}
