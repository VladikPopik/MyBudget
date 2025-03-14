import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import LoginPage from './pages/LoginPage.jsx';
import RegisterPage from "./pages/RegisterPage.jsx";
import HomePage from "./pages/HomePage.jsx";
import BudgetPage from './pages/BudgetPage.jsx';
import "./App.css";


function App() {
  return (
    <Router>
      <Routes>
      <Route path="/" element={<LoginPage />} />
      <Route path="/login" element={<LoginPage />} />
      <Route path="/register" element={<RegisterPage />} />
      <Route path="/home" element={<HomePage />}/>
      <Route path="/budget" element={< BudgetPage/>}/>
      </Routes>
    </Router>
  );
}

export default App;
