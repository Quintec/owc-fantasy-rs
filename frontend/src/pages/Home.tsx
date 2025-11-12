
import { useAuth } from '../contexts/AuthContext'
import draftDemo from '../assets/draftdemo.png'
import scoring from '../assets/scoring.png'



export default function Home() {
  const { user, loading } = useAuth()

  const apiBaseUrl = import.meta.env.VITE_API_URL || "http://localhost:8080";

  const handleLogin = () => {
    window.location.href = `${apiBaseUrl}/api/auth/login`
  }

  return (
    <div className="flex justify-center flex-col items-center p-10">
        <div className="text-center">
            <h1 className="text-6xl font-bold text-white">OWC Fantasy</h1>
            <p className="text-3xl text-gray-400 pt-5 max-w-2xl ">Draft your very own OWC team and score points based on your team's individual performance!</p>
         </div>
         <div className="w-full flex justify-center">
            {loading ? null : user ? (
              <div className="text-center">
                <h2 className="text-2xl text-white m-4">Welcome back, {user.username}!</h2>
              </div>
            ) : (
              <button 
                className="bg-purple-500 text-white px-4 py-2 my-5 rounded-md text-2xl"
                onClick={handleLogin}
              >
                Login with osu!
              </button>
            )}
          </div>
        <div className="flex md:flex-row flex-col bg-purple-700 max-w-3/4 m-5 p-5 md:p-8 items-center rounded-lg gap-6 md:gap-8">
              <p className="text-white text-xl md:flex-1">Each week, you get the chance to put together your own team from 8 OWC players. You'll have a budget of $100M to spend, and no more than two picks from the same country. You'll even get to choose a captain whose points are doubled for that week.</p>
              <img src={draftDemo} className="w-full md:w-1/2 max-w-md h-auto object-contain flex-shrink-0" alt="Draft demo"></img>
        </div>

        <div className="flex md:flex-row flex-col-reverse bg-purple-700 max-w-3/4 m-5 p-5 md:p-8 items-center rounded-lg gap-6 md:gap-8">
              <img src={scoring} className="w-full md:w-1/2 max-w-md h-auto object-contain flex-shrink-0" alt="Scoring explanation"></img>
              <p className="text-white text-xl md:flex-1">At the end of each week, the points for your team are tallied based on their individual performances in their matches. Every week you will get the chance to draft a new team with updated prices based on the performance of players from the last round.</p>
        </div>
    </div>
  )
}