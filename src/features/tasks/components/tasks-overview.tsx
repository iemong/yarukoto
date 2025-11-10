import { Loader2, PlusIcon } from "lucide-react";
import { Button } from "@/components/ui/button";
import { useTasks } from "@/features/tasks/hooks/use-tasks";
import { cn } from "@/lib/utils/cn";
import type { Task } from "@/types/tasks";

const TaskRow = ({ task }: { task: Task }) => (
	<article
		className={cn(
			"flex items-start justify-between rounded-lg border bg-card px-4 py-3",
			task.completed && "opacity-60",
		)}
	>
		<div>
			<p className="font-medium">{task.title}</p>
			{task.notes ? (
				<p className="text-sm text-muted-foreground">{task.notes}</p>
			) : null}
		</div>
		{task.due ? (
			<span className="text-xs text-muted-foreground">
				期日: {new Date(task.due).toLocaleDateString("ja-JP")}
			</span>
		) : null}
	</article>
);

export const TasksOverview = () => {
	const { data, isLoading } = useTasks();

	if (isLoading) {
		return (
			<div className="flex h-48 items-center justify-center text-muted-foreground">
				<Loader2 className="mr-2 h-4 w-4 animate-spin" />
				タスクを読込中...
			</div>
		);
	}

	return (
		<div className="space-y-4">
			<header className="flex items-center justify-between">
				<div>
					<h2 className="text-2xl font-semibold">今日のタスク</h2>
					<p className="text-sm text-muted-foreground">
						Google Tasks API と同期する前のスタブデータです。
					</p>
				</div>
				<Button variant="primary">
					<PlusIcon className="mr-2 h-4 w-4" />
					新規タスク
				</Button>
			</header>
			<div className="space-y-3">
				{(data ?? []).map((task) => (
					<TaskRow key={task.id} task={task} />
				))}
			</div>
		</div>
	);
};
