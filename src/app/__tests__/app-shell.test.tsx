import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { TasksOverview } from "@/features/tasks/components/tasks-overview";

const createTestClient = () =>
	new QueryClient({
		defaultOptions: {
			queries: {
				retry: false,
				retryDelay: 0,
			},
		},
	});

describe("TasksOverview", () => {
	it("renders placeholder task list", async () => {
		const client = createTestClient();

		render(
			<QueryClientProvider client={client}>
				<TasksOverview />
			</QueryClientProvider>,
		);

		expect(await screen.findByText("今日のタスク")).toBeInTheDocument();
	});
});
