import { createRoot } from "react-dom/client";
import "./index.css";

import { BrowserRouter, Routes, Route } from "react-router-dom";
import { Header } from "./components";
import { Game, Home } from "./pages";

createRoot(document.getElementById("root")!).render(
  <BrowserRouter>
    <Header />
    <main>
      <Routes>
        <Route path="/" element={<Home />} />
        <Route path="/play" element={<Game />} />
      </Routes>
    </main>
  </BrowserRouter>,
);
