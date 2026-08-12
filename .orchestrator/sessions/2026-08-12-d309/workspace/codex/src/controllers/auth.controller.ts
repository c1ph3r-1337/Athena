import type { Request, Response, NextFunction } from 'express';
import type { AuthService } from '../services/auth.service.js';

type Credentials = { email: string; password: string };

export class AuthController {
  constructor(private readonly authService: AuthService) {}

  register = async (req: Request<unknown, unknown, Credentials>, res: Response, next: NextFunction): Promise<void> => {
    try {
      const result = await this.authService.register(req.body.email, req.body.password);
      res.status(201).json({ data: result });
    } catch (error) { next(error); }
  };

  login = async (req: Request<unknown, unknown, Credentials>, res: Response, next: NextFunction): Promise<void> => {
    try {
      const result = await this.authService.login(req.body.email, req.body.password);
      res.status(200).json({ data: result });
    } catch (error) { next(error); }
  };
}
