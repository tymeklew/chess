import { ReactNode, useEffect, useState } from "react";
import { Board, ChatBox } from "../../components";
import { GiCrossedPistols, GiFlyingFlag, GiChessKing , GiRobotAntennas , GiPerson, GiPlayButton, GiChessQueen } from "react-icons/gi";
import { fromUCI, Move, Piece, toUCI } from "../shared/interfaces";
import "./game.css";
import { Colour, PieceType } from "../shared/enum";
import DEFAULT_BOARD from "../shared/const";
import { BsRobot } from "react-icons/bs";

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
  const [side , setSide] = useState<Colour>(Colour.White);
  const [promotionPiece , setPromotionPiece] = useState<PieceType | null>(null);
  const [myTurn , setMyTurn] = useState<boolean>(false);
  const [comp , setComp] = useState<boolean>(false);

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
      setComp(false);
      url = `ws://localhost:5173/api/ws/play?game=bot&side=white&difficulty=${difficulty}`
    } else {
      setComp(true);
      url = "ws://localhost:5173/api/ws/play?game=competitive"
    }
    const socket = new WebSocket(url);
    setWebSock(socket);
    // Connection opened
    socket.onopen = () => {
      setBoard(DEFAULT_BOARD);
      setStatus(Status.Connected);

      socket.send(JSON.stringify({type : "legal_moves"}))
    };
    socket.onclose = () => {setStatus(Status.Disconnected)};

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
    // Check for castling which is when the king moves 2 squares
    if (fromPiece.type == PieceType.King && Math.abs(mv.from[0] - mv.to[0]) == 2) {
      // Move the rook
      // First check if the rook is on the right or left
      let rookFile = mv.to[0] == 6 ? 7 : 0;
      let rookRank = mv.from[1];
      let rook = temp[rookRank][rookFile];
      temp[rookRank][rookFile] = null;
      temp[mv.from[1]][mv.from[0]] = null;
      temp[mv.to[1]][mv.to[0]] = fromPiece;
      temp[mv.to[1]][mv.to[0] - 1] = rook;
    }else {
    temp[mv.to[1]][mv.to[0]] = fromPiece;
    temp[mv.from[1]][mv.from[0]] = null;
    if (mv.promotion) {
      temp[mv.to[1]][mv.to[0]] = {
        colour: fromPiece.colour,
        type: mv.promotion
      };
    }
  }
    webSock?.send(JSON.stringify({type: "move", data: toUCI(mv)}));
    webSock?.send(JSON.stringify({type: "legal_moves"}));
    setBoard(temp);
    setMyTurn(false);
  }

  function handleMessage(evt : any) { 
    let msg = JSON.parse(evt.data);
    console.log("Handling message : " + msg.type + " : ");
    console.log(board);
    switch (msg.type) {
      case "move":
        console.log(msg.data);
        let move = fromUCI(msg.data);
        console.log(move);
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
          setMyTurn(true);
          return temp;
        });
        break;
      case "legal_moves":
        console.log("Recieved legal moves");
        console.log(msg.data);
        let mvs = msg.data.split(",").filter((m: string) => m !== "").map((move : string) => fromUCI(move)); 
        setLegalMoves(mvs);
        break;
      case "game_over":
        alert("Game Over : " + msg.data);
        setComp(false);
        break;
      case "game_started":
        console.log("Game started : " + msg.data)
        switch (msg.data) {
          case "white":
            setSide(Colour.White);
            setMyTurn(true);
            break;
          case "black":
            setSide(Colour.Black);
            setMyTurn(false);
            break;
        }
        alert("Game started playing as " + msg.data);
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
        <Board board={board} moves={legalMoves} setActiveSquare={setActiveSquare} activeSquare={activeSquare} sendMove={sendMove} setPromotionPiece={setPromotionPiece} side={side} myTurn={myTurn}/>
        <div className="info-container">
           {getStatus()}
          <h2> <GiCrossedPistols/> </h2>
          <button className="play-button" onClick={() => handleButtonClick(null)}> 
            Play 
          </button>
          <h2> <BsRobot/></h2>
          <button onClick={() => handleButtonClick("easy")}>
             Easy
          </button>
          <button onClick={() => handleButtonClick("medium")}>
             Medium 
          </button>
          <button onClick={() => handleButtonClick("hard")}>
            Hard 
          </button>
          <button onClick={() => {
            webSock?.close();
            setStatus(Status.Disconnected);
          }}> Quit </button>
        </div>
      </div>
    </div>
  );
}
