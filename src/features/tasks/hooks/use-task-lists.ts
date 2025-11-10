import { useQuery } from "@tanstack/react-query";
import { taskService } from "@/features/tasks/api/task-service";
import type { TaskList } from "@/types/tasks";

const taskListQueryKey = ["task-lists"];

export const useTaskLists = () =>
	useQuery<TaskList[]>({
		queryKey: taskListQueryKey,
		queryFn: () => taskService.listTaskLists(),
		staleTime: 1000 * 60,
	});
