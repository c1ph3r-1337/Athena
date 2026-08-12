import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

async function main() {
  await prisma.user.create({
    data: {
      email: 'admin@example.com',
      name: 'Admin User',
    },
  });

  await prisma.resource.create({
    data: {
      name: 'Initial Resource',
      description: 'This is a seeded resource',
      price: 19.99,
    },
  });
}

main()
  .catch((e) => {
    console.error(e);
    process.exit(1);
  })
  .finally(async () => {
    await prisma.$disconnect();
  });
