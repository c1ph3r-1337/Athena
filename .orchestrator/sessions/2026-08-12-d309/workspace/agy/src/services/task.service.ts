import prisma from '../config/database';
import { Prisma } from '@prisma/client';

export const createTask = async (data: Prisma.TaskCreateInput) => {
  return await prisma.task.create({ data });
};

export const getTasks = async (page: number = 1, limit: number = 10) => {
  const skip = (page - 1) * limit;
  const [tasks, total] = await Promise.all([
    prisma.task.findMany({ skip, take: limit, orderBy: { createdAt: 'desc' } }),
    prisma.task.count(),
  ]);
  
  return {
    data: tasks,
    meta: {
      total,
      page,
      limit,
      totalPages: Math.ceil(total / limit),
    },
  };
};

export const getTaskById = async (id: string) => {
  return await prisma.task.findUnique({ where: { id } });
};

export const updateTask = async (id: string, data: Prisma.TaskUpdateInput) => {
  return await prisma.task.update({
    where: { id },
    data,
  });
};

export const deleteTask = async (id: string) => {
  return await prisma.task.delete({ where: { id } });
};
