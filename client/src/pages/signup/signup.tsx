import React, { useState } from "react";
import {Link, redirect } from "react-router-dom"
import "./signup.css"

export default function Signup() {
    const [email, setEmail] = useState("");
    const [username, setUsername] = useState("");
    const [password, setPassword] = useState("");

    const PASSWORD_REGEX = /(^(?=.*[A-Z]).*(?=.*[a-z]).*(?=.*[\d]).*(?=.*[.!?@%^&*\(\)\{\}\[\]]).*){8,72}/
    const EMAIL_REGEX = /^[A-Za-z0-9._%+-]{1,64}@[A-Za-z0-9.-]{1,253}\.[A-Za-z]{2,}$/;
    const USERNAME_REGEX = /.{3,20}/;

    function handleSubmit(e : React.FormEvent) {
        e.preventDefault();

        if (!PASSWORD_REGEX.test(password)) {
            alert("Password must be between 8 and 72 characters and contain at least one uppercase letter, one lowercase letter, one number and one special character");
            return;
        }else if (!EMAIL_REGEX.test(email)) {
            alert("Invalid Email");
            return;
        }else if (!USERNAME_REGEX.test(username)) {
            alert("Username must be between 3 and 20 characters");
            return;
        }

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
                <Link to = "/login"> Already have an account? Login </Link>
            </form>
         </div>
}