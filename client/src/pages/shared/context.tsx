import { createContext, useContext, useEffect, useState } from "react";
import React from "react";
import Cookies from "js-cookie";

interface AuthContextType {
    loggedIn: boolean;
    setLoggedIn: React.Dispatch<React.SetStateAction<boolean>>;
    logOut: () => void;
}

const AuthContext = createContext<AuthContextType | null>(null);

interface AuthProviderProps {
    children: React.ReactNode;
}

const AuthProvider: React.FC<AuthProviderProps> = ({children}) => {
    const [loggedIn , setLoggedIn] = useState(false);
    
    useEffect(() => {
        const id = Cookies.get("session_id");
        if (id) {
            setLoggedIn(true);
        }
        // Add call to server to check if session is valid
    } , []);

    function logOut() {
        Cookies.remove("session_id");
        setLoggedIn(false);
    }

    return (
        <AuthContext.Provider value={{loggedIn , setLoggedIn , logOut}}>
            {children}
        </AuthContext.Provider>
    );

}

export default AuthProvider;
export const useAuth = () => {
    return useContext(AuthContext);
}
