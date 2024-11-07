import { ReactNode, useState } from "react";
import { Board, ChatBox } from "../../components";
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
    socket.onopen = () => setStatus(Status.Connected);
    socket.onclose = () => setStatus(Status.Disconnected);

    socket.onmessage = handleMessage;
  }

  function handleMessage(evt) {
    console.log(evt.data as string);
    setMessages((oldMessages) => [...oldMessages, evt.data as string]);
    console.log(messages.length);
  }

  function sendMessage(msg: string) {
    console.log("b");
    if (webSock == undefined) return;
    console.log("sending message");
    webSock.send(msg);
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
      <button onClick={handleButtonClick}> Play </button>
      <div className="board-container">
        {getStatus()}
        <Board />
      </div>
      <ChatBox messages={messages} sendMessage={sendMessage} />
    </div>
  );
}
