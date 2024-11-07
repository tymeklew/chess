import { useState } from "react";

interface ChatProps {
  messages: string[];
  sendMessage: (msg: string) => void;
}
export default function ChatBox(props: ChatProps) {
  const [messageValue, setMessageValue] = useState("");
  return (
    <div>
      Hello
      {props.messages.map((msg) => (
        <p key={Math.random()}>{msg}</p>
      ))}
      <input
        type="text"
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
