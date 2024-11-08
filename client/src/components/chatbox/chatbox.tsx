import { useState } from "react";
import "./chatbox.css";

interface ChatProps {
  messages: string[];
  sendMessage: (msg: string) => void;
}
export default function ChatBox(props: ChatProps) {
  const [messageValue, setMessageValue] = useState("");
  return (
    <div className="chatbox-container">
      <div className="message-container">
        {props.messages.map((msg) => (
          <p key={Math.random()} className="message">
            {msg}
          </p>
        ))}
      </div>
      <input
        type="text"
        placeholder="Press Enter to send"
        onChange={(e) => setMessageValue(e.target.value)}
        onKeyDown={(e) => {
          if (e.key == "Enter") {
            props.sendMessage(messageValue);
          }
        }}
      />
    </div>
  );
}
