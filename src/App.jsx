import { useState } from "react";
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import LoginPage from './pages/LoginPage.jsx';
import RegisterPage from "./pages/RegisterPage.jsx";

function App() {
  // const [greetMsg, setGreetMsg] = useState("");
  // const [name, setName] = useState("");

  // async function greet() {
  //   // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  //   setGreetMsg(await invoke("greet", { name }));
  // }

  return (
    <Router>
      <Routes>
      <Route path="/" element={<LoginPage />} />
      <Route path="/login" element={<LoginPage />} />
      <Route path="/register" element={<RegisterPage />} />
      </Routes>
    </Router>
  );
}

export default App;
