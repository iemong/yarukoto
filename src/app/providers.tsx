import { QueryClientProvider } from "@tanstack/react-query";
import { ReactQueryDevtools } from "@tanstack/react-query-devtools";
import { RouterProvider } from "@tanstack/react-router";
import { RouterDevtools } from "@tanstack/router-devtools";
import { useState } from "react";
import { createAppRouter } from "@/app/router";
import { createAppQueryClient } from "@/lib/tanstack/query-client";

export const AppProviders = () => {
	const [queryClient] = useState(() => createAppQueryClient());
	const [router] = useState(() => createAppRouter(queryClient));

	return (
		<QueryClientProvider client={queryClient}>
			<RouterProvider router={router} />
			{import.meta.env.DEV ? (
				<>
					<ReactQueryDevtools
						initialIsOpen={false}
						buttonPosition="bottom-left"
					/>
					<RouterDevtools position="bottom-right" />
				</>
			) : null}
		</QueryClientProvider>
	);
};
