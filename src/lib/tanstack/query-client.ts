import { QueryClient } from "@tanstack/react-query";

export const createAppQueryClient = () =>
	new QueryClient({
		defaultOptions: {
			queries: {
				staleTime: 1000 * 30,
				refetchOnWindowFocus: false,
				retry: 1,
				gcTime: 1000 * 60 * 5,
			},
			mutations: {
				retry: 1,
			},
		},
	});
