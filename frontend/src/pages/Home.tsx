
import { useAuth } from '../contexts/AuthContext'



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
                <h2 className="text-2xl text-white mb-4">Welcome back, {user.username}!</h2>
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
        <div className="flex flex-row justify-between bg-purple-500 max-w-3/4 m-5 p-5 items-center">
              <p className="text-white text-2xl">Each week, you get the chance to put together your own squad of 8 players. You’ll have a budget of $100M to spend, and no more than two picks from the same country, so every decision counts. To make it even more interesting, you’ll choose a captain whose points are doubled for that week.</p>
              <img src="https://a.ppy.sh/2" className="min-w-1/4 m-10"></img>
        </div>

        <div className="flex flex-row justify-between bg-purple-500 max-w-3/4 m-5 p-5 items-center">
              <img src="https://a.ppy.sh/2" className="min-w-1/4 m-10"></img>
              <p className="text-white text-2xl">Points come from how your players perform in their real matches. Goals, saves, wins, losses—it all matters. But there’s a catch: if a team plays twice in the same weekend, those players only earn half points for that round. It keeps the playing field level and makes you think twice about loading up on certain picks.</p>
        </div>
    </div>
  )
}