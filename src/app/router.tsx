import type { QueryClient } from "@tanstack/react-query";
import { createRouter } from "@tanstack/react-router";
import type { RouterContext } from "@/app/router-context";
import { routeTree } from "@/app/routes";

export const createAppRouter = (queryClient: QueryClient) =>
	createRouter({
		routeTree,
		context: { queryClient } satisfies RouterContext,
	});

declare module "@tanstack/react-router" {
	interface Register {
		router: ReturnType<typeof createAppRouter>;
	}
}
