import type { Task, TaskList } from "@/types/tasks";

const SAMPLE_LISTS: TaskList[] = [
	{ id: "inbox", title: "Inbox", taskCount: 3 },
	{ id: "today", title: "Today", taskCount: 2 },
	{ id: "next", title: "Next", taskCount: 5 },
];

const SAMPLE_TASKS: Task[] = [
	{
		id: "task-1",
		title: "Google OAuth フローのモック",
		completed: false,
		due: new Date().toISOString(),
		notes: "Rust 側の oauth モジュール設計を固める",
	},
	{
		id: "task-2",
		title: "TanStack Router ルート定義",
		completed: true,
	},
];

export const taskService = {
	async listTaskLists(): Promise<TaskList[]> {
		return Promise.resolve(SAMPLE_LISTS);
	},
	async listTasks(): Promise<Task[]> {
		return Promise.resolve(SAMPLE_TASKS);
	},
};
