import { ListChecks, Loader2 } from "lucide-react";
import { useTaskLists } from "@/features/tasks/hooks/use-task-lists";
import { cn } from "@/lib/utils/cn";

export const TaskListPanel = () => {
	const { data, isLoading } = useTaskLists();

	if (isLoading) {
		return (
			<div className="flex h-full items-center justify-center text-muted-foreground">
				<Loader2 className="mr-2 h-4 w-4 animate-spin" />
				タスクリストを読込中...
			</div>
		);
	}

	return (
		<nav className="space-y-1 p-4 text-sm">
			{(data ?? []).map((list) => (
				<button
					key={list.id}
					className={cn(
						"flex w-full items-center justify-between rounded-md px-3 py-2 text-left transition",
						"hover:bg-sidebar-accent hover:text-sidebar-foreground",
					)}
					type="button"
				>
					<span className="flex items-center gap-2">
						<ListChecks className="h-4 w-4" />
						{list.title}
					</span>
					<span className="text-muted-foreground">{list.taskCount}</span>
				</button>
			))}
		</nav>
	);
};
