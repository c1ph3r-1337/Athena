import type { ErrorRequestHandler, RequestHandler } from 'express';
import { ZodError, type ZodType } from 'zod';
import { CustomError } from '../utils/customError.js';

export const notFoundHandler: RequestHandler = (req, _res, next) =>
  next(new CustomError(404, `Route ${req.method} ${req.originalUrl} was not found.`, 'NOT_FOUND'));

/** Validates one request location and replaces it with Zod's sanitized output. */
export const validate = <T>(schema: ZodType<T>, location: 'body' | 'params' | 'query' = 'body'): RequestHandler =>
  (req, _res, next) => {
    const result = schema.safeParse(req[location]);
    if (!result.success) return next(result.error);
    // Express's Request types make these locations readonly-ish, but Express permits assigning them at runtime.
    Object.assign(req[location], result.data);
    return next();
  };

export const errorHandler: ErrorRequestHandler = (error, _req, res, _next) => {
  if (error instanceof ZodError) {
    return res.status(400).json({ error: { code: 'VALIDATION_ERROR', message: 'Request validation failed.', details: error.flatten() } });
  }
  const known = error instanceof CustomError ? error : new CustomError(500, 'An unexpected error occurred.');
  if (known.statusCode >= 500) console.error(error);
  return res.status(known.statusCode).json({ error: { code: known.code, message: known.message, ...(known.details === undefined ? {} : { details: known.details }) } });
};
