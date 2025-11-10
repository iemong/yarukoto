import { Outlet } from "@tanstack/react-router";
import { TaskListPanel } from "@/features/tasks/components/task-list-panel";

export const AppLayout = () => (
	<div className="grid min-h-screen grid-rows-[auto,1fr] bg-background text-foreground">
		<header className="flex items-center justify-between border-b px-6 py-4">
			<div>
				<p className="text-xs uppercase tracking-[0.3em] text-muted-foreground">
					Yarukoto
				</p>
				<h1 className="text-xl font-semibold">Google Tasks の新しい習慣</h1>
			</div>
			<div className="text-sm text-muted-foreground">
				Apple Silicon / Google OAuth 2.0
			</div>
		</header>
		<div className="grid grid-cols-1 gap-0 md:grid-cols-[240px,1fr]">
			<aside className="border-b border-r bg-sidebar p-0 md:border-b-0">
				<TaskListPanel />
			</aside>
			<main className="p-6">
				<Outlet />
			</main>
		</div>
	</div>
);
