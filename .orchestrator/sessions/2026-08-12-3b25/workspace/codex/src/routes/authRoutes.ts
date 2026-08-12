import { Router } from 'express';
import { AuthController } from '../controllers/authController';
import { AuthService } from '../services/authService';
import { authRateLimit } from '../middleware/security';

export const createAuthRouter = (authService: AuthService): Router => {
  const router = Router();
  const controller = new AuthController(authService);
  router.post('/register', authRateLimit, controller.register);
  router.post('/login', authRateLimit, controller.login);
  router.post('/refresh', authRateLimit, controller.refresh);
  return router;
};
