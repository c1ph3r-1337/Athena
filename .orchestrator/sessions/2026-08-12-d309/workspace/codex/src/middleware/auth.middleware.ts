import type { NextFunction, Request, Response, RequestHandler } from 'express';
import cors, { type CorsOptions } from 'cors';
import rateLimit from 'express-rate-limit';
import helmet from 'helmet';
import { verifyAccessToken } from '../utils/jwt.util.js';
import { CustomError } from '../utils/customError.js';

declare global {
  namespace Express {
    interface Request {
      auth?: { userId: string; email: string };
    }
  }
}

export const requireAuth = (req: Request, _res: Response, next: NextFunction): void => {
  const [scheme, token] = req.get('authorization')?.split(/\s+/) ?? [];
  if (scheme !== 'Bearer' || !token) return next(new CustomError(401, 'Bearer authentication is required.', 'AUTH_REQUIRED'));
  try {
    const payload = verifyAccessToken(token);
    req.auth = { userId: payload.sub, email: payload.email };
    next();
  } catch (error) {
    next(error);
  }
};

export const authenticationRateLimiter = rateLimit({
  windowMs: 15 * 60 * 1000,
  limit: 10,
  standardHeaders: 'draft-8',
  legacyHeaders: false,
  message: { error: { code: 'RATE_LIMITED', message: 'Too many authentication attempts. Please try again later.' } }
});

export const apiRateLimiter = rateLimit({
  windowMs: 15 * 60 * 1000,
  limit: 300,
  standardHeaders: 'draft-8',
  legacyHeaders: false
});

export const createSecurityMiddleware = (allowedOrigins: string[] = []): RequestHandler[] => {
  const corsOptions: CorsOptions = {
    origin(origin, callback) {
      if (!origin || allowedOrigins.includes(origin)) return callback(null, true);
      callback(new CustomError(403, 'Origin is not allowed by CORS policy.', 'CORS_DENIED'));
    },
    credentials: true,
    methods: ['GET', 'POST', 'PUT', 'PATCH', 'DELETE', 'OPTIONS'],
    allowedHeaders: ['Authorization', 'Content-Type'],
    maxAge: 86400
  };
  return [helmet(), cors(corsOptions), apiRateLimiter];
};
