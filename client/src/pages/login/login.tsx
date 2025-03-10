import React, { useState } from "react";
import { useAuth } from "../shared/context";
export default function Login() {
    const [identifier, setIdentifier] = useState("");
    const [password, setPassword] = useState("");

    const auth = useAuth();
    function handleSubmit(e : React.FormEvent) {
        e.preventDefault();

        fetch("/api/auth/login" , {
            method : "POST",
            headers : {
                "Content-Type" : "application/json"
            },
            body : JSON.stringify({
                identifier,
                password
            }),
            credentials : "include"
        })

        auth.setLoggedIn(true);
    }
    return <div className="form-container">
        <form onSubmit={handleSubmit}>
            <h1> Login </h1>
            <input placeholder="Username or Email" value={identifier} onChange={(e) => setIdentifier(e.target.value)}/>
            <input type="password" placeholder="Password" value={password} onChange={(e) => setPassword(e.target.value)}/>
            <button type="submit">Login</button>
        </form>
    </div>;
}