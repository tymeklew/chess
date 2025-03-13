import { createRoot } from "react-dom/client";
import "./index.css";

import { BrowserRouter, Routes, Route } from "react-router-dom";
import { Header } from "./components";
import { Friends, Game, Home , Login , Signup  , Me} from "./pages";
import AuthProvider from "./pages/shared/context";

createRoot(document.getElementById("root")!).render(
  <BrowserRouter>
  <AuthProvider>
    <Header />
    <main>
      <Routes>
        <Route path="/" element={<Home />} />
        <Route path="/play" element={<Game />} />
        <Route path="/login" element={<Login/>} />
        <Route path="/signup" element={<Signup/>} />
        <Route path="/friends" element={<Friends/>} />
        <Route path="/me" element={<Me/>} />
        <Route path="*" element={<div>Not Found</div>} />
      </Routes>
    </main>
    </AuthProvider>
  </BrowserRouter>,
);
