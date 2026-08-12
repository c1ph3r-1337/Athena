export class CustomError extends Error {
  public readonly statusCode: number;
  public readonly code: string;
  public readonly details?: unknown;
  public readonly isOperational = true;

  constructor(statusCode: number, message: string, code = 'INTERNAL_ERROR', details?: unknown) {
    super(message);
    this.name = 'CustomError';
    this.statusCode = statusCode;
    this.code = code;
    this.details = details;
    Error.captureStackTrace?.(this, CustomError);
  }
}
