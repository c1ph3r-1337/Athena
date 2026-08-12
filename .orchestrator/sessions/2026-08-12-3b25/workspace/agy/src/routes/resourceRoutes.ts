import { Router } from 'express';
import { ResourceController } from '../controllers/resourceController';

const router = Router();
const resourceController = new ResourceController();

router.post('/', resourceController.createResource.bind(resourceController));
router.get('/', resourceController.getResources.bind(resourceController));
router.get('/:id', resourceController.getResourceById.bind(resourceController));
router.put('/:id', resourceController.updateResource.bind(resourceController));
router.delete('/:id', resourceController.deleteResource.bind(resourceController));

export default router;
