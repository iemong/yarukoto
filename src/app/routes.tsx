import {
	createRootRouteWithContext,
	createRoute,
} from "@tanstack/react-router";
import type { RouterContext } from "@/app/router-context";
import { AppLayout } from "@/app/views/app-layout";
import { TasksOverview } from "@/features/tasks/components/tasks-overview";

const rootRoute = createRootRouteWithContext<RouterContext>()({
	component: AppLayout,
});

const dashboardRoute = createRoute({
	getParentRoute: () => rootRoute,
	path: "/",
	component: TasksOverview,
});

export const routeTree = rootRoute.addChildren([dashboardRoute]);
