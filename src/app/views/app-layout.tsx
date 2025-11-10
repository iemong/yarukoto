import { Outlet } from "@tanstack/react-router";
import { Button } from "@/components/ui/button";
import { useAuthSession } from "@/features/auth/hooks/use-auth-session";
import { TaskListPanel } from "@/features/tasks/components/task-list-panel";

const AuthControls = () => {
	const { sessionQuery, loginMutation } = useAuthSession();
	const isAuthenticated = sessionQuery.data?.isAuthenticated ?? false;
	const statusLabel = sessionQuery.isLoading
		? "状態確認中..."
		: isAuthenticated
			? "Google 連携済み"
			: "未ログイン";

	return (
		<div className="flex items-center gap-3">
			<span className="text-sm text-muted-foreground">{statusLabel}</span>
			<Button
				variant="secondary"
				size="sm"
				disabled={loginMutation.isPending}
				onClick={() => loginMutation.mutate()}
			>
				{loginMutation.isPending
					? "接続中..."
					: isAuthenticated
						? "再認証"
						: "Googleでログイン"}
			</Button>
		</div>
	);
};

export const AppLayout = () => (
	<div className="grid min-h-screen grid-rows-[auto,1fr] bg-background text-foreground">
		<header className="flex flex-col gap-4 border-b px-6 py-4 md:flex-row md:items-center md:justify-between">
			<div>
				<p className="text-xs uppercase tracking-[0.3em] text-muted-foreground">
					Yarukoto
				</p>
				<h1 className="text-xl font-semibold">Google Tasks の新しい習慣</h1>
			</div>
			<div className="flex flex-col gap-2 text-sm text-muted-foreground md:items-end">
				<span>Apple Silicon / Google OAuth 2.0</span>
				<AuthControls />
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
