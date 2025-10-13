import { BrowserRouter as Router, Routes, Route } from 'react-router-dom'
import Home from './pages/Home'
import Navbar from './components/Navbar'
import Team from './pages/Team'
import Leaderboard from './pages/Leaderboard'
import { AuthProvider } from './contexts/AuthContext'

function App() {

  return (
    <AuthProvider>
      <Router>
      <Navbar />
      <main className="flex-1">
        <Routes>
          <Route path="/" element={<Home />} />
          <Route path="/team" element={<Team />} />
          <Route path="/leaderboard" element={<Leaderboard/>} />
        </Routes>
      </main>
      </Router>
    </AuthProvider>
  )
}

export default App
