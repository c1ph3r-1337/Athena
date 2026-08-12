import { NextFunction, Request, Response } from 'express';
import { AuthError, AuthService, Role } from '../services/authService';

export class AuthController {
  constructor(private readonly authService: AuthService) {}

  register = async (req: Request, res: Response, next: NextFunction): Promise<void> => {
    try {
      // Public registration must never allow callers to choose a privileged role.
      const result = await this.authService.register(req.body?.email, req.body?.password, 'user' as Role);
      res.status(201).json(result);
    } catch (error) { next(error); }
  };

  login = async (req: Request, res: Response, next: NextFunction): Promise<void> => {
    try {
      const result = await this.authService.login(req.body?.email, req.body?.password);
      res.status(200).json(result);
    } catch (error) { next(error); }
  };

  refresh = async (req: Request, res: Response, next: NextFunction): Promise<void> => {
    try {
      const refreshToken = req.body?.refreshToken;
      if (typeof refreshToken !== 'string' || !refreshToken) throw new AuthError(400, 'refreshToken is required');
      res.status(200).json(await this.authService.refresh(refreshToken));
    } catch (error) { next(error); }
  };
}
