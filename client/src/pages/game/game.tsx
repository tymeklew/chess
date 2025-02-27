import { ReactNode, useEffect, useState } from "react";
import { Board, ChatBox } from "../../components";
import { GiCrossedPistols, GiFlyingFlag, GiChessKing } from "react-icons/gi";
import { Move, Piece } from "../shared/interfaces";
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
    console.log(mv);
    let temp = board;
    temp[mv.to[1]][mv.to[0]] = temp[mv.from[1]][mv.from[0]];
    temp[mv.from[1]][mv.from[0]] = null;
    setBoard(temp);
    webSock?.send(JSON.stringify({type : "move" , data : `${ALPHABET[mv.from[0]]}${8 - mv.from[1]}${ALPHABET[mv.to[0]]}${8 - mv.to[1]}`}));
    webSock?.send(JSON.stringify({type : "legal"}))
  }

  function handleMessage(evt : any) {
    let msg = JSON.parse(evt.data);
    console.log(msg);
    switch (msg.type) {
      case "CHAT":
        setMessages([...messages , evt.data]);
        break;
      case "legal_moves":

      console.log(msg.data);
      let legalMoves : Move[] = [];
        (msg.data.split(",")).forEach(element => {
            let from = element.substring(0,2);
            let to = element.substring(2,4); 

            let fromX = ALPHABET.indexOf(from[0]);
            let fromY = 8 - parseInt(from[1]);
            let toX = ALPHABET.indexOf(to[0]);
            let toY = 8 - parseInt(to[1]);

            legalMoves.push({from : [fromX , fromY] , to : [toX , toY]});
        });

        setLegalMoves(legalMoves);
        break;
        case "move":
          let move = msg.data;
          let from = move.substring(0,2);
          let to = move.substring(2,4);
          let fromX = ALPHABET.indexOf(from[0]);
          let fromY = 8 - parseInt(from[1]);
          let toX = ALPHABET.indexOf(to[0]);
          let toY = 8 - parseInt(to[1]);

          let temp = board;
          temp[toY][toX] = temp[fromY][fromX];
          temp[fromY][fromX] = null;
          setBoard(temp);
          break;
    }
  }

  function sendMessage(msg: string) {
    webSock?.send(msg); 
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
          <ChatBox messages={messages} sendMessage={sendMessage} />
        </div>
      </div>
      {/*<ChatBox messages={messages} sendMessage={sendMessage} />*/}
    </div>
  );
}
