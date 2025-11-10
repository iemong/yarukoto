export type Task = {
	id: string;
	title: string;
	completed: boolean;
	due?: string;
	notes?: string;
};

export type TaskList = {
	id: string;
	title: string;
	taskCount: number;
};
