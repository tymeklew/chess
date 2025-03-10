import React, { useEffect } from "react"
import { useAuth } from "../shared/context"
import { FaPlusCircle  , FaMinusCircle} from "react-icons/fa";
import "./friends.css"

interface User {
    username : string;
    user_id : string;
}

interface FriendRequests {
    request_id : string;
    username : string;
    incoming : boolean;
    status : string;
}

export default function Friends() {
    const auth = useAuth();
    const [username , setUsername] = React.useState("");
    const [users , setUsers] = React.useState<User[]>([]);
    const [friends , setFriends] = React.useState<User[]>([]);
    const [friendRequests , setFriendRequests] = React.useState<FriendRequests[]>([]);

    useEffect(() => {
        fetch("/api/friends/list" , {
            method : "GET"
        }).then(res => res.json()).then(data => {
            console.log(data);
            setFriends(data);
        })

        fetch("/api/friends/requests/list" , {
            method : "GET"
        }).then(res => res.json()).then(data => {
            console.log(data);
            setFriendRequests(data);
        }
        )
    } , [])

    function handleChange(val : string) {
        setUsername(val);

        fetch(`/api/friends/search?username=${val}` , {
            method : "GET",
        }).then(res => res.json()).then(data => {
            console.log(data);
            setUsers(data);
        })

    }

    function sendFriendRequest(id : string) {
        console.log(id);
        let test = JSON.stringify({
            friend_id : id
        })
        console.log(test);
        fetch("/api/friends/request" , {
            method : "POST",
            headers : {
                "Content-Type" : "application/json"
            },
            body : JSON.stringify({
                "friend_id" : id
            })
        }).then(res => {
            if (res.status === 201) {
                alert("Friend Request Sent");
            } else if (res.status == 409) {
                alert("Friend Request Already Sent");
            } 
            else {
                alert("Friend Request Failed");
            }
        })
    }
    return <div className="friends-container"> 
        <h1> Friends </h1>
        <div className="inner-friends-container"> 
            <div className="friends-list">

            </div>
            <div className="friend-requests-container">
                <h2> Friend Requests </h2>
                {friendRequests.map(request => {
                    return <div key={request.request_id} className="friend-request">
                        <p className="friend-request-incoming"> {request.incoming ? "Incoming" : "Outgoing"} : {request.username}</p>
                        <p className={`friend-request-status ${request.status}`}>{request.status} </p>
                        {request.incoming ? <>
                        <FaPlusCircle />
                        <FaMinusCircle /> </> : <> </>} 
                    </div>
                })}
            </div>
            <div className="add-friend">
                <input placeholder="Username" value={username} onChange={(e) => handleChange(e.target.value)}/>
                {users.map(user => <div key={user.user_id} className="search-result"> {user.username} <FaPlusCircle className="friend-add" onClick={() => sendFriendRequest(user.user_id)}/> </div>)}
            </div>
        </div>
    </div>
}