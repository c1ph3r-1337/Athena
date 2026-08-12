import 'dotenv/config';
import bcrypt from 'bcryptjs';
import { PrismaClient, Role, TaskStatus } from '@prisma/client';

const prisma = new PrismaClient();

async function main(): Promise<void> {
  const email = (process.env.SEED_ADMIN_EMAIL ?? 'admin@example.com').toLowerCase();
  const password = process.env.SEED_ADMIN_PASSWORD ?? 'ChangeMe123!';
  const passwordHash = await bcrypt.hash(password, 12);

  const admin = await prisma.user.upsert({
    where: { email },
    update: {
      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
      passwordHash,
      role: Role.ADMIN,
    },
    create: {
      email,
      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
      passwordHash,
      role: Role.ADMIN,
    },
  });

  const starterTaskCount = await prisma.task.count({ where: { ownerId: admin.id } });
  if (starterTaskCount === 0) {
    await prisma.task.create({
      data: {
        title: 'Welcome to the task API',
        description: 'This seed task can be safely removed.',
        status: TaskStatus.TODO,
        ownerId: admin.id,
      },
    });
  }

  console.info(`Seeded administrator: ${admin.email}`);
}

main()
  .catch((error: unknown) => {
    console.error('Database seed failed:', error);
    process.exitCode = 1;
  })
  .finally(async () => {
    await prisma.$disconnect();
  });
