import { z } from 'zod';

const taskId = z.object({ id: z.string().uuid() });
const taskFields = {
  title: z.string().trim().min(1).max(200),
  description: z.string().trim().max(5000).optional(),
  completed: z.boolean().optional(),
  dueDate: z.string().datetime({ offset: true }).optional()
};

export const taskParamsSchema = taskId;
export const createTaskSchema = z.object({ title: taskFields.title, description: taskFields.description, dueDate: taskFields.dueDate });
export const updateTaskSchema = z.object(taskFields).partial().refine((value) => Object.keys(value).length > 0, 'At least one field is required.');
