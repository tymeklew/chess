import { ReactNode, useState } from "react";
import { Board, ChatBox } from "../../components";
import { GiCrossedPistols, GiFlyingFlag, GiChessKing } from "react-icons/gi";
import "./game.css";

enum Status {
  Disconnected,
  Connecting,
  Connected,
}

enum Colour {
  Black,
  White,
}

interface Pieces {
  colour: Colour;
  position: [number, number];
}

export default function Game() {
  const [messages, setMessages] = useState<string[]>([]);
  const [webSock, setWebSock] = useState<WebSocket>();
  const [status, setStatus] = useState<Status>(Status.Disconnected);
  const [pieces, setPieces] = useState<Pieces[]>([]);

  function handleButtonClick() {
    setStatus(Status.Connecting);
    const socket = new WebSocket("ws://localhost:3000/ws");
    setWebSock(socket);
    // Connection opened
    socket.onopen = () => {
      socket.send(
        JSON.stringify({
          type: "game_state",
        }),
      );
      setStatus(Status.Connected);
    };
    socket.onclose = () => setStatus(Status.Disconnected);

    socket.onmessage = handleMessage;
  }

  function handleMessage(evt) {
    const info = JSON.parse(evt.data);
    console.log(info);
    switch (info["type"]) {
      case "message":
        setMessages((oldMessages) => [
          ...oldMessages,
          evt.data["data"] as string,
        ]);
        break;
      case "game_state":
        updateBoard(info["data"] as string);
        break;
    }
  }

  function updateBoard(data: string) {
    if (data == undefined) return;
    console.log(data);
    let pieces = [];
    data.split(";").forEach((c) => {
      console.log(c);
    });
  }

  function sendMessage(msg: string) {
    if (webSock == undefined) return;
    webSock.send(
      JSON.stringify({
        type: "message",
        data: msg,
      }),
    );
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
        <Board />
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
