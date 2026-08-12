import cors, { CorsOptions } from 'cors';
import { Request, Response } from 'express';
import rateLimit from 'express-rate-limit';
import helmet from 'helmet';

const configuredOrigins = (process.env.CORS_ORIGIN ?? '').split(',').map((origin) => origin.trim()).filter(Boolean);

export const corsOptions: CorsOptions = {
  origin(origin, callback) {
    // Allow non-browser clients; browsers with an Origin must be explicitly approved.
    if (!origin || configuredOrigins.includes(origin)) return callback(null, true);
    return callback(new Error('Origin not allowed by CORS'));
  },
  credentials: true,
  methods: ['GET', 'POST', 'PUT', 'PATCH', 'DELETE', 'OPTIONS'],
  allowedHeaders: ['Authorization', 'Content-Type'],
  maxAge: 86400,
};

export const securityHeaders = helmet({ crossOriginResourcePolicy: { policy: 'same-site' } });
export const corsMiddleware = cors(corsOptions);

export const apiRateLimit = rateLimit({
  windowMs: 15 * 60 * 1000,
  limit: Number(process.env.RATE_LIMIT_MAX ?? 300),
  standardHeaders: 'draft-7',
  legacyHeaders: false,
  message: { error: 'Too many requests, please try again later.' },
});

export const authRateLimit = rateLimit({
  windowMs: 15 * 60 * 1000,
  limit: Number(process.env.AUTH_RATE_LIMIT_MAX ?? 10),
  standardHeaders: 'draft-7',
  legacyHeaders: false,
  skipSuccessfulRequests: true,
  message: { error: 'Too many authentication attempts, please try again later.' },
});

/** Reject oversized JSON bodies before parsing them. Use before express.json(). */
export const rejectLargeJson = (maxBytes = 1024 * 1024) => (req: Request, res: Response, next: () => void): void => {
  const contentLength = Number(req.get('content-length') ?? 0);
  if (Number.isFinite(contentLength) && contentLength > maxBytes) {
    res.status(413).json({ error: 'Request entity too large' });
    return;
  }
  next();
};
