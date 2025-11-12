import axios from 'axios';

const API_BASE_URL = import.meta.env.VITE_API_URL || 'http://localhost:8080';

export interface ImportPScoresRequest {
  pscore_text: string;
  round: string;
}

export interface ImportPScoresResponse {
  updated_count: number;
  skipped_eliminated: string[];
  skipped_not_found: string[];
  errors: string[];
}

export async function importPScores(
  pscoreText: string,
  round: string
): Promise<ImportPScoresResponse> {
  const response = await axios.post<ImportPScoresResponse>(
    `${API_BASE_URL}/api/players/import_pscores`,
    {
      pscore_text: pscoreText,
      round,
    },
    {
      withCredentials: true,
    }
  );
  return response.data;
}

export interface SetDefaultPricesResponse {
  updated_count: number;
  errors: string[];
}

export async function setDefaultPrices(
  round: string
): Promise<SetDefaultPricesResponse> {
  const response = await axios.post<SetDefaultPricesResponse>(
    `${API_BASE_URL}/api/players/set_default_prices`,
    {
      round,
    },
    {
      withCredentials: true,
    }
  );
  return response.data;
}
