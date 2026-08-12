import { Request, Response, NextFunction } from 'express';
import * as taskService from '../services/taskService';
import { z } from 'zod';

const createTaskSchema = z.object({
  title: z.string().min(1).max(200),
  description: z.string().optional(),
  status: z.enum(['TODO', 'IN_PROGRESS', 'DONE']).optional(),
});

const updateTaskSchema = z.object({
  title: z.string().min(1).max(200).optional(),
  description: z.string().optional(),
  status: z.enum(['TODO', 'IN_PROGRESS', 'DONE']).optional(),
});

export const createTask = async (req: Request, res: Response, next: NextFunction) => {
  try {
    const validatedData = createTaskSchema.parse(req.body);
    const userId = (req as any).user.userId;
    const task = await taskService.createTask(userId, validatedData);
    res.status(201).json({ success: true, data: task, message: 'Task created' });
  } catch (err: any) {
    if (err instanceof z.ZodError) {
      res.status(400).json({ success: false, message: 'Validation error', errors: (err as any).errors });
    } else {
      next(err);
    }
  }
};

export const getTasks = async (req: Request, res: Response, next: NextFunction) => {
  try {
    const userId = (req as any).user.userId;
    const tasks = await taskService.getTasksByUser(userId);
    res.status(200).json({ success: true, data: tasks });
  } catch (err) {
    next(err);
  }
};

export const getTask = async (req: Request, res: Response, next: NextFunction) => {
  try {
    const userId = (req as any).user.userId;
    const { id } = req.params;
    const task = await taskService.getTaskById(userId, id as string);
    res.status(200).json({ success: true, data: task });
  } catch (err) {
    next(err);
  }
};

export const updateTask = async (req: Request, res: Response, next: NextFunction) => {
  try {
    const validatedData = updateTaskSchema.parse(req.body);
    const userId = (req as any).user.userId;
    const { id } = req.params;
    const task = await taskService.updateTask(userId, id as string, validatedData);
    res.status(200).json({ success: true, data: task, message: 'Task updated' });
  } catch (err: any) {
    if (err instanceof z.ZodError) {
      res.status(400).json({ success: false, message: 'Validation error', errors: (err as any).errors });
    } else {
      next(err);
    }
  }
};

export const deleteTask = async (req: Request, res: Response, next: NextFunction) => {
  try {
    const userId = (req as any).user.userId;
    const { id } = req.params;
    await taskService.deleteTask(userId, id as string);
    res.status(204).send();
  } catch (err) {
    next(err);
  }
};
