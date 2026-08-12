import { NextFunction, Request, Response } from 'express';
import { AuthError, AuthService, Role } from '../services/authService';

export interface AuthenticatedRequest extends Request {
  auth?: { userId: string; email: string; role: Role };
}

/** Requires a valid bearer access token and exposes its trusted claims on req.auth. */
export const authenticate = (authService: AuthService) => (req: AuthenticatedRequest, _res: Response, next: NextFunction): void => {
  const header = req.get('authorization');
  if (!header?.startsWith('Bearer ')) return next(new AuthError(401, 'Bearer token is required'));
  const token = header.slice(7).trim();
  if (!token) return next(new AuthError(401, 'Bearer token is required'));
  try {
    const claims = authService.verifyAccessToken(token);
    req.auth = { userId: claims.sub, email: claims.email, role: claims.role };
    next();
  } catch (error) {
    next(error);
  }
};

/** Apply after authenticate. A user needs one of the listed roles to continue. */
export const authorize = (...roles: Role[]) => (req: AuthenticatedRequest, _res: Response, next: NextFunction): void => {
  if (!req.auth) return next(new AuthError(401, 'Authentication is required'));
  if (!roles.includes(req.auth.role)) return next(new AuthError(403, 'You do not have permission to perform this action'));
  next();
};
