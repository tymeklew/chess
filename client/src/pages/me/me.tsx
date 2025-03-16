import { useEffect, useState } from "react";
import "./me.css"

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
            console.log(temp);
            setBotGames(temp);
        });
        fetch(COMPETITIVE_URL).then(res => res.json()).then(data => {
            let temp = data.map((game : any) => {
                return {
                    user_id : game.user_id,
                    username : game.username,
                    started : new Date(game.started),
                    finished : new Date(game.finished),
                    winner : game.winner,
                    white : game.white
                }
            })
            setCompetitiveGames(temp);
        })
    } , [])
    return (
        <div>
            <h1>Hello {username}</h1>
            <p>Your email is {email} </p> 

            <div className="games">
            <div className="game-container">
            <h2> Bot Games </h2>
            {
               botGames.map((game : BotGame) => {
                    return <div className="bot-game game" key={game.id}>
                        <h1> {game.won ? 'Won' : 'Lost'} </h1>
                        <p> Against <b>{game.difficulty}</b> bot as <b>{game.white ? 'White' : 'Black'}</b></p>
                    </div>
               }) 
            }
            </div>
            <div className="game-container">
            <h2> Comp Games </h2>
            {
                competitiveGames.map((game : PlayerGame) => {
                    let duration_seconds = Math.floor((game.finished.getTime() - game.started.getTime()) / 1000);
                    let duration_minutes = Math.floor(duration_seconds / 60);
                    return <div className="competitive-game game" key={Math.random()}>
                        <h1> {game.winner == game.user_id ? 'Won' : 'Lost'} </h1> 
                        <p> As <b>{game.white ? 'White' : 'Black'}</b> against <b>{game.username}</b> taking {duration_minutes}:{duration_seconds}</p>
                    </div>
                })
            }
            </div>
            </div>
        </div>
    )
}