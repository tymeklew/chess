import { useEffect, useState } from "react";

interface BotGame {
    id : string;
    difficulty : string;
    white : boolean;
    won: boolean;
}

interface PlayerGame {
    user_id : string;
    username : string;
    started : Date;
    finished : Date;
    winner : string;
    white : boolean;
}
export default function Me() {
    const [username , setUsername] = useState<string>("");
    const [email , setEmail] = useState<string>("");
    const [botGames , setBotGames] = useState<BotGame[]>([]);
    const [competitiveGames , setCompetitiveGames] = useState<PlayerGame[]>([]);

    const ME_URL = "/api/me"
    const BOT_URL = "/api/games/bot"
    const COMPETITIVE_URL = "/api/games/competitive"
    useEffect(() => {
        console.log("REAL");
        fetch(ME_URL).then(res => res.json()).then(data => {
            setUsername(data.username)
            setEmail(data.email)
        })
        fetch(BOT_URL).then(res => res.json()).then(data => {
            let temp = data.map((game : any) => {
                let difficulty = "easy";
                if (game.difficulty == 2) {
                    difficulty = "medium";
                } else if (game.difficulty == 3) {
                    difficulty = "hard";
                }

                return {
                    id : game.id,
                    difficulty : difficulty,
                    side : game.side,
                    won : game.won
                }
            })
            setBotGames(temp);
        });
        fetch(COMPETITIVE_URL).then(res => res.json()).then(data => {
            setCompetitiveGames(data);
        })
    } , [])
    return (
        <div>
            <h1>Hello {username}</h1>
            <p>Your email is {email} </p> 

            <h2> Bot Games </h2>
            {
               botGames.map((game : BotGame) => {
                    return <div className="bot-game game" key={game.id}>
                        <p> Difficulty : {game.difficulty}</p>
                        <p> {game.won ? "Won" : "Lost"} playing as {game.white ? "Black" : "White"} </p>
                    </div>
               }) 
            }
            <h2> Comp Games </h2>
            {
                competitiveGames.map((game : PlayerGame) => {
                    return <div className="competitive-game game" key={game.user_id}>
                        <p> {game.winner == game.user_id ? "Won" : "Lost"} playing as {game.white ? "Black" : "White"} against {game.username} </p>
                    </div>
                })
            }
        </div>
    )
}