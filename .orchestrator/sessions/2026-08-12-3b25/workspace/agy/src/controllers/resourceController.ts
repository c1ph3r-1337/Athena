import { Request, Response, NextFunction } from 'express';
import { ResourceService } from '../services/resourceService';
import { createResourceSchema, updateResourceSchema, queryResourceSchema } from '../validators/resourceSchema';

const resourceService = new ResourceService();

export class ResourceController {
  async createResource(req: Request, res: Response, next: NextFunction) {
    try {
      const validatedData = createResourceSchema.parse(req.body);
      const resource = await resourceService.createResource(validatedData);
      res.status(201).json(resource);
    } catch (error) {
      next(error);
    }
  }

  async getResourceById(req: Request, res: Response, next: NextFunction) {
    try {
      const { id } = req.params;
      const resource = await resourceService.getResourceById(id);
      if (!resource) {
        return res.status(404).json({ error: 'Resource not found' });
      }
      res.status(200).json(resource);
    } catch (error) {
      next(error);
    }
  }

  async getResources(req: Request, res: Response, next: NextFunction) {
    try {
      const query = queryResourceSchema.parse(req.query);
      const result = await resourceService.getResources(query);
      res.status(200).json(result);
    } catch (error) {
      next(error);
    }
  }

  async updateResource(req: Request, res: Response, next: NextFunction) {
    try {
      const { id } = req.params;
      const validatedData = updateResourceSchema.parse(req.body);
      const resource = await resourceService.updateResource(id, validatedData);
      res.status(200).json(resource);
    } catch (error) {
      next(error);
    }
  }

  async deleteResource(req: Request, res: Response, next: NextFunction) {
    try {
      const { id } = req.params;
      await resourceService.deleteResource(id);
      res.status(204).send();
    } catch (error) {
      next(error);
    }
  }
}
