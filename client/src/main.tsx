import { createRoot } from "react-dom/client";
import "./index.css";

import { BrowserRouter, Routes, Route } from "react-router-dom";
import { Header } from "./components";
import { Game, Home , Login , Signup } from "./pages";

createRoot(document.getElementById("root")!).render(
  <BrowserRouter>
    <Header />
    <main>
      <Routes>
        <Route path="/" element={<Home />} />
        <Route path="/play" element={<Game />} />
        <Route path="/login" element={<Login/>} />
        <Route path="/signup" element={<Signup/>} />
        <Route path="*" element={<div>Not Found</div>} />
      </Routes>
    </main>
  </BrowserRouter>,
);
