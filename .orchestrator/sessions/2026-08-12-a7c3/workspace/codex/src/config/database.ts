import { PrismaClient } from '@prisma/client';

/**
 * A single Prisma client is shared by the application. Keeping it on globalThis
 * prevents duplicate connection pools during development hot reloads.
 */
const prismaGlobal = globalThis as unknown as { prisma?: PrismaClient };

export const prisma =
  prismaGlobal.prisma ??
  new PrismaClient({
    log: process.env.NODE_ENV === 'development' ? ['warn', 'error'] : ['error'],
  });

if (process.env.NODE_ENV !== 'production') {
  prismaGlobal.prisma = prisma;
}

/** Closes open database connections for graceful application shutdown. */
export const disconnectDatabase = async (): Promise<void> => {
  await prisma.$disconnect();
};

export default prisma;
