import React, { useState } from "react";
import {redirect } from "react-router-dom"
import "./signup.css"

export default function Signup() {
    const [email, setEmail] = useState("");
    const [username, setUsername] = useState("");
    const [password, setPassword] = useState("");

    function handleSubmit(e : React.FormEvent) {
        e.preventDefault();

        fetch("/api/auth/signup" , {
            method : "POST",
            headers : {
                "Content-Type" : "application/json"
            },
            body : JSON.stringify({
                email,
                username,
                password
            })
        }).then(res => {
            if (res.status === 201) {
                alert("Sign Up Successfull");
                return redirect("/login");
            } else {
                alert("Sign Up Failed");
            }
        }
        )
    }
    return <div className="form-container"> 
            <form onSubmit={handleSubmit}> 
                <h1> Sign Up </h1>
                <input type="email" placeholder="Email" value={email} onChange={(e) => setEmail(e.target.value)}/>
                <input placeholder="Username" value={username} onChange={(e) => setUsername(e.target.value)}/>
                <input type="password" placeholder="Password" value={password} onChange={(e) => setPassword(e.target.value)}/>
                <button type="submit">Sign Up</button>
            </form>
         </div>
}