import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { listen } from "@tauri-apps/api/event";
import { useEffect } from "react";
import { fetchSession, startOAuth } from "@/features/auth/api/commands";

const sessionQueryKey = ["session-status"];

export const useAuthSession = () => {
	const queryClient = useQueryClient();

	useEffect(() => {
		const unlistenPromise = listen("auth://completed", () => {
			queryClient.invalidateQueries({ queryKey: sessionQueryKey });
		});
		return () => {
			unlistenPromise.then((unlisten) => unlisten());
		};
	}, [queryClient]);

	const sessionQuery = useQuery({
		queryKey: sessionQueryKey,
		queryFn: fetchSession,
		staleTime: 1000 * 60 * 5,
	});

	const loginMutation = useMutation({
		mutationFn: async () => {
			await startOAuth();
		},
	});

	return {
		sessionQuery,
		loginMutation,
	};
};
