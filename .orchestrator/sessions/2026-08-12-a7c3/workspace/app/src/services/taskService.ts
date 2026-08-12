import { prisma } from '../config/database';

export const createTask = async (userId: string, data: any) => {
  return await prisma.task.create({
    data: {
      ...data,
      ownerId: userId,
    },
  });
};

export const getTasksByUser = async (userId: string) => {
  return await prisma.task.findMany({
    where: { ownerId: userId },
    orderBy: { createdAt: 'desc' },
  });
};

export const getTaskById = async (userId: string, taskId: string) => {
  const task = await prisma.task.findFirst({
    where: { id: taskId, ownerId: userId },
  });

  if (!task) {
    const error: any = new Error('Task not found');
    error.status = 404;
    throw error;
  }
  return task;
};

export const updateTask = async (userId: string, taskId: string, data: any) => {
  const task = await prisma.task.findFirst({
    where: { id: taskId, ownerId: userId },
  });

  if (!task) {
    const error: any = new Error('Task not found');
    error.status = 404;
    throw error;
  }

  return await prisma.task.update({
    where: { id: taskId },
    data,
  });
};

export const deleteTask = async (userId: string, taskId: string) => {
  const task = await prisma.task.findFirst({
    where: { id: taskId, ownerId: userId },
  });

  if (!task) {
    const error: any = new Error('Task not found');
    error.status = 404;
    throw error;
  }

  await prisma.task.delete({
    where: { id: taskId },
  });
};
