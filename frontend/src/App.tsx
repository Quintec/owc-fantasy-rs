import { BrowserRouter as Router, Routes, Route } from 'react-router-dom'
import Home from './pages/Home'
import Navbar from './components/Navbar'
import Footer from './components/Footer'
import Team from './pages/Team'
import Leaderboard from './pages/Leaderboard'
import Scores from './pages/Scores'
import Admin from './pages/Admin'
import NotFound from './pages/NotFound'
import { AuthProvider } from './contexts/AuthContext'

function App() {

  return (
    <AuthProvider>
      <Router>
      <div className="flex flex-col min-h-screen">
      <Navbar />
      <main className="flex-1">
        <Routes>
          <Route path="/" element={<Home />} />
          <Route path="/team" element={<Team />} />
          <Route path="/leaderboard" element={<Leaderboard/>} />
          <Route path="/scores" element={<Scores/>} />
          <Route path="/admin" element={<Admin/>} />
          <Route path="*" element={<NotFound/>} />
        </Routes>
      </main>
      <Footer />
      </div>
      </Router>
    </AuthProvider>
  )
}

export default App
