import { useState, useEffect } from 'react';
import { useRound } from '../contexts/RoundContext';
import { parseMultiplayerLinks, eliminatePlayers, unEliminatePlayers, getAllPlayers, importPlayersFromParticipants } from '../api/players';
import { importPScores } from '../api/pscores';
import type { PlayerProps } from '../types';
import { useAuth } from '../contexts/AuthContext';

export default function Admin() {
  const { round: currentRound } = useRound();
  const { user } = useAuth();
  const [selectedRound, setSelectedRound] = useState('ro16'); // Default to ro16
  const [linksText, setLinksText] = useState('');
  const [participantsText, setParticipantsText] = useState('');
  const [pscoreText, setPscoreText] = useState('');
  const [status, setStatus] = useState<string | null>(null);
  const [country, setCountry] = useState<string>('');
  const [players, setPlayers] = useState<PlayerProps[]>([]);

  // Update selected round when current round changes
  useEffect(() => {
    setSelectedRound(currentRound);
  }, [currentRound]);

  const submitLinks = async () => {
    try {
      setStatus('Submitting links...');
      // Parse input - can be either full URLs or just match IDs
      const links = linksText.split('\n')
        .map(l => l.trim())
        .filter(Boolean)
        .filter(l => l !== 'link') // Filter out the word "link"
        .map(l => {
          // If it's just a number (match ID), convert to full URL
          if (/^\d+$/.test(l)) {
            return `https://osu.ppy.sh/community/matches/${l}`;
          }
          // Otherwise assume it's already a full URL
          return l;
        });
      const res = await parseMultiplayerLinks(links, selectedRound);

      // Notify how many links were added
      setStatus(`Added ${links.length} multiplayer links`);

      // Refresh players and sort by score
      const updated = await getAllPlayers();
      setPlayers(updated);

      // Also append parse response for debugging if available
      if (res && typeof res === 'object') {
        // leave status as the Added message, but also log res
        console.debug('parseMultiplayerLinks response:', res);
      }
    } catch (err: any) {
      setStatus('Error: ' + (err?.message || String(err)));
    }
  };

  const eliminateTeam = async () => {
    try {
      setStatus('Eliminating players...');
      const res = await eliminatePlayers(country || undefined);
      setStatus('Eliminated player ids: ' + JSON.stringify(res.eliminated) + (country ? ` (filtered by country=${country})` : ''));
    } catch (err: any) {
      setStatus('Error: ' + (err?.message || String(err)));
    }
  };

  // load players initially
  useEffect(() => {
    let mounted = true;
    (async () => {
      try {
        const p = await getAllPlayers();
        if (!mounted) return;
        setPlayers(p);
      } catch (e) {
        console.error('Failed to load players for admin page', e);
      }
    })();
    return () => { mounted = false; };
  }, []);

  if (!user) return <div className="p-5 text-white">You must be logged in to use admin tools.</div>;

  const importPlayers = async () => {
    try {
      setStatus('Sending participants text to backend for import...');
      
      // Backend handles OAuth and DB insertion
      const result = await importPlayersFromParticipants(participantsText);
      
      setStatus(`Successfully imported ${result.count} players!${result.errors.length > 0 ? ` Errors: ${result.errors.slice(0, 3).join(', ')}` : ''}`);
      
      // Refresh players list
      const updated = await getAllPlayers();
      setPlayers(updated);
      setParticipantsText(''); // Clear the textarea
    } catch (err: any) {
      const errorMsg = err?.response?.data?.error || err?.message || String(err);
      setStatus('Error importing players: ' + errorMsg);
    }
  };

  const importPlayerPrices = async () => {
    try {
      setStatus('Importing player prices from pScores...');
      
      const result = await importPScores(pscoreText, selectedRound);
      
      // Only show "not found" players, not eliminated ones (that's intentional)
      const notFoundCount = result.skipped_not_found?.length || 0;
      
      setStatus(
        `Successfully updated prices for ${result.updated_count} players!` +
        (notFoundCount > 0 ? `\nNot found in database (${notFoundCount}): ${result.skipped_not_found.join(', ')}` : '') +
        (result.errors?.length > 0 ? `\nErrors: ${result.errors.join(', ')}` : '')
      );
      
      setPscoreText(''); // Clear the textarea
    } catch (err: any) {
      const errorMsg = err?.response?.data?.error || err?.message || String(err);
      setStatus('Error importing pScores: ' + errorMsg);
    }
  };

  return (
    <div className="p-6 text-white">
      <h1 className="text-2xl font-bold mb-4">Admin</h1>

      <div className="mb-4">
        <label className="mr-2 font-semibold">Round:</label>
        <select 
          value={selectedRound} 
          onChange={(e) => setSelectedRound(e.target.value)}
          className="bg-gray-800 text-white px-3 py-1 rounded border border-gray-600"
        >
          <option value="ro64">RO64</option>
          <option value="ro32">RO32</option>
          <option value="ro16">RO16</option>
          <option value="qf">Quarterfinals</option>
          <option value="sf">Semifinals</option>
          <option value="f">Finals</option>
          <option value="gf">Grand Finals</option>
        </select>
        <span className="ml-4 text-sm text-gray-400">Current round: {currentRound}</span>
      </div>

      {/* Import Players Section */}
      <div className="mb-6 p-4 bg-gray-800 rounded">
        <h2 className="text-xl font-semibold mb-2">Import Players from Tournament</h2>
        <p className="text-sm text-gray-400 mb-2">
          Paste the participants table from the osu! wiki (markdown format).
          The system will extract player IDs, fetch their data via OAuth2, and add them to the database.
        </p>
        <label className="block mb-1">Participants markdown:</label>
        <textarea 
          value={participantsText} 
          onChange={(e) => setParticipantsText(e.target.value)} 
          className="w-full bg-gray-900 p-2 rounded h-60 font-mono text-sm"
          placeholder="Paste markdown table here, e.g.:&#10;| ::{ flag=US }:: | **United States** | **[player1](https://osu.ppy.sh/users/123)**, [player2](https://osu.ppy.sh/users/456) |"
        />
        <button onClick={importPlayers} className="mt-2 bg-blue-600 px-4 py-2 rounded hover:bg-blue-700">
          Import Players
        </button>
      </div>

      {/* Import pScores Section */}
      <div className="mb-6 p-4 bg-gray-800 rounded">
        <h2 className="text-xl font-semibold mb-2">Updates Player Prices from pScores {`(for ${selectedRound})`}</h2>
        <p className="text-sm text-gray-400 mb-2">
          Ensure the data you use is the p-scores from the previous round (e.g. use p-scores from ro16 to update qf).
          Paste player pScore data (username followed by pScore, one per line).
          The system will calculate prices and update the database for the current round.
          Players without pScores will automatically use prices from the previous round or a rank-based default.
        </p>
        <label className="block mb-1">pScore data:</label>
        <textarea 
          value={pscoreText} 
          onChange={(e) => setPscoreText(e.target.value)} 
          className="w-full bg-gray-900 p-2 rounded h-60 font-mono text-sm"
          placeholder="Paste pScore data here, e.g.:&#10;scylla	1.692&#10;Raikouhou	1.659&#10;[MG]Arnold24x24	1.649"
        />
        <button onClick={importPlayerPrices} className="mt-2 bg-green-600 px-4 py-2 rounded hover:bg-green-700">
          Import pScores & Calculate Prices 
        </button>
      </div>

      <div className="mb-4">
        <label className="block mb-1">Multiplayer links (one per line or match IDs):</label>
        <textarea 
          value={linksText} 
          onChange={(e) => setLinksText(e.target.value)} 
          className="w-full bg-gray-900 p-2 rounded h-40 font-mono text-sm"
          placeholder="Can be full URLs or just match IDs:&#10;https://osu.ppy.sh/community/matches/119720145&#10;or just:&#10;119720145&#10;119768390"
        />
        <button onClick={submitLinks} className="mt-2 bg-purple-600 px-4 py-2 rounded">Parse links</button>
      </div>

      <div className="mb-4">
        <label className="block mb-1">Eliminate players in tournament:</label>
        <input placeholder="Optional country code (e.g. US)" value={country} onChange={e => setCountry(e.target.value.toUpperCase())} className="bg-gray-900 p-2 rounded mr-2 w-100" />
        <button onClick={eliminateTeam} className="bg-red-600 px-4 py-2 rounded mr-2">Eliminate players</button>
        <button onClick={async () => {
          try {
            setStatus('Un-eliminating players...');
            const res = await unEliminatePlayers(country || undefined);
            setStatus('Un-eliminated player ids: ' + JSON.stringify(res.uneliminated) + (country ? ` (filtered by country=${country})` : ''));
          } catch (err: any) {
            setStatus('Error: ' + (err?.message || String(err)));
          }
        }} className="bg-green-600 px-4 py-2 rounded">Un-eliminate players</button>
      </div>

      {status && (
        <div className="mt-4 p-2 bg-gray-800 rounded">
          <pre className="whitespace-pre-wrap">{status}</pre>
        </div>
      )}

      <div className="mt-6">
        <h2 className="text-xl font-semibold mb-2">Players (sorted by score)</h2>
        <div className="overflow-auto max-h-96 bg-gray-900 p-2 rounded">
          <table className="w-full table-auto">
            <thead>
              <tr className="text-left text-sm text-gray-400">
                <th className="px-2">#</th>
                <th className="px-2">Player</th>
                <th className="px-2">Country</th>
                <th className="px-2">Score</th>
              </tr>
            </thead>
            <tbody>
              {[...players].sort((a, b) => (b.score ?? 0) - (a.score ?? 0)).map((p, idx) => {
                return (
                  <tr key={p.id} className="border-t border-gray-800 text-white text-sm">
                    <td className="px-2 py-2">{idx + 1}</td>
                    <td className="px-2 py-2 flex items-center gap-2">
                      <img src={`https://a.ppy.sh/${p.id}`} className="h-8 w-8 rounded-full" alt="avatar" />
                      <span>{p.username}</span>
                    </td>
                    <td className="px-2 py-2">{p.country}</td>
                    <td className="px-2 py-2">{p.score ?? 0}</td>
                  </tr>
                );
              })}
            </tbody>
          </table>
        </div>
      </div>
    </div>
  );
}
