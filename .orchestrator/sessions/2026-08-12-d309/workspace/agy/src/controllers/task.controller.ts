import { Request, Response, NextFunction } from 'express';
import * as TaskService from '../services/task.service';

export const createTask = async (req: Request, res: Response, next: NextFunction) => {
  try {
    const task = await TaskService.createTask(req.body);
    res.status(201).json(task);
  } catch (error) {
    next(error);
  }
};

export const getTasks = async (req: Request, res: Response, next: NextFunction) => {
  try {
    const page = parseInt(req.query.page as string) || 1;
    const limit = parseInt(req.query.limit as string) || 10;
    const tasks = await TaskService.getTasks(page, limit);
    res.status(200).json(tasks);
  } catch (error) {
    next(error);
  }
};

export const getTaskById = async (req: Request, res: Response, next: NextFunction) => {
  try {
    const task = await TaskService.getTaskById(req.params.id as string);
    if (!task) {
      return res.status(404).json({ message: 'Task not found' });
    }
    res.status(200).json(task);
  } catch (error) {
    next(error);
  }
};

export const updateTask = async (req: Request, res: Response, next: NextFunction) => {
  try {
    const task = await TaskService.updateTask(req.params.id as string, req.body);
    res.status(200).json(task);
  } catch (error) {
    next(error);
  }
};

export const deleteTask = async (req: Request, res: Response, next: NextFunction) => {
  try {
    await TaskService.deleteTask(req.params.id as string);
    res.status(204).send();
  } catch (error) {
    next(error);
  }
};
