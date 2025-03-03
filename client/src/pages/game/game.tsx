import { ReactNode, useEffect, useState } from "react";
import { Board, ChatBox } from "../../components";
import { GiCrossedPistols, GiFlyingFlag, GiChessKing , GiRobotAntennas , GiPerson } from "react-icons/gi";
import { fromUCI, Move, Piece, toUCI } from "../shared/interfaces";
import "./game.css";
import { Colour, PieceType } from "../shared/enum";
import DEFAULT_BOARD from "../shared/const";
import Promotion from "../../components/promotion/promotion";

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

  useEffect(() => {
    setBoard(DEFAULT_BOARD)
  } , []);

  function handleButtonClick() {
    setStatus(Status.Connecting);
    const socket = new WebSocket("ws://localhost:5173/api/ws/bot");
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
    console.log(toUCI(mv));
    webSock?.send(JSON.stringify({type : "move" , move : toUCI(mv)}))
  }

  function handleMessage(evt : any) { 
    console.log(evt);
    let msg = JSON.parse(evt.data);
    switch (msg.type) {
      case "move":
        let move = fromUCI(msg.data);
        let temp = board;
        temp[move.to[1]][move.to[0]] = temp[move.from[1]][move.from[0]];
        temp[move.from[1]][move.from[0]] = null;
        setBoard(temp);
        break;
      case "legal_moves":
        let mvs = msg.data.split(",").map((move : string) => fromUCI(move)); 
        console.log(mvs);
        setLegalMoves(mvs);
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
        <Board board={board} moves={legalMoves} setActiveSquare={setActiveSquare} activeSquare={activeSquare} sendMove={sendMove}/>
        <div className="info-container">
          {getStatus()}
          <button onClick={handleButtonClick}>
            {" "}
            Play <GiChessKing />{" "}
          </button>
          <button>
            {" "}
            Draw <GiCrossedPistols />
          </button>
          <button>
            {" "}
            Resign <GiFlyingFlag />{" "}
          </button>
          {/*<ChatBox messages={messages} sendMessage={sendMessage} />*/}
        </div>
      </div>
      {/*<ChatBox messages={messages} sendMessage={sendMessage} />*/}
    </div>
  );
}
