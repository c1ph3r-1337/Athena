import prisma from '../config/database';

export class ResourceService {
  async createResource(data: any) {
    return prisma.resource.create({ data });
  }

  async getResourceById(id: string) {
    return prisma.resource.findUnique({ where: { id } });
  }

  async getResources(query: any) {
    const { page = 1, limit = 10, sortBy = 'createdAt', sortOrder = 'desc' } = query;
    const skip = (page - 1) * limit;

    const [data, total] = await Promise.all([
      prisma.resource.findMany({
        skip,
        take: limit,
        orderBy: { [sortBy]: sortOrder },
      }),
      prisma.resource.count(),
    ]);

    return {
      data,
      meta: {
        total,
        page,
        limit,
        totalPages: Math.ceil(total / limit),
      },
    };
  }

  async updateResource(id: string, data: any) {
    return prisma.resource.update({ where: { id }, data });
  }

  async deleteResource(id: string) {
    return prisma.resource.delete({ where: { id } });
  }
}
