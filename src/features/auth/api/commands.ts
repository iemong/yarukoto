import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-opener";

export type AuthStartResponse = {
	authorizationUrl: string;
};

type SessionStatusResponse = {
	is_authenticated: boolean;
};

export type SessionStatus = {
	isAuthenticated: boolean;
};

export const startOAuth = async () => {
	const { authorizationUrl } = await invoke<AuthStartResponse>("start_oauth");
	await open(authorizationUrl);
};

export const fetchSession = async (): Promise<SessionStatus> => {
	const payload = await invoke<SessionStatusResponse>("current_session");
	return {
		isAuthenticated: payload.is_authenticated,
	};
};
