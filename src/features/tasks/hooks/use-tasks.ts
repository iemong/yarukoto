import { useQuery } from "@tanstack/react-query";
import { taskService } from "@/features/tasks/api/task-service";
import type { Task } from "@/types/tasks";

const taskQueryKey = ["tasks"];

export const useTasks = () =>
	useQuery<Task[]>({
		queryKey: taskQueryKey,
		queryFn: () => taskService.listTasks(),
		staleTime: 1000 * 30,
	});
