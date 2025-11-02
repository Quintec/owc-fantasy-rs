import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import './index.css'
import App from './App.tsx'
import { AuthProvider } from './contexts/AuthContext'
import { RoundProvider } from './contexts/RoundContext'

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <AuthProvider>
      <RoundProvider>
        <App />
      </RoundProvider>
    </AuthProvider>
  </StrictMode>,
)
