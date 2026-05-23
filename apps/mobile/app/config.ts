// API base URL for the Techub Comms server.
// In production, this points to the Railway-deployed backend.
// Override via EXPO_PUBLIC_API_URL environment variable.
const API_BASE_URL =
  process.env.EXPO_PUBLIC_API_URL || 'https://api.thbtechub.sbs';

export const config = {
  apiBaseUrl: API_BASE_URL,
};
