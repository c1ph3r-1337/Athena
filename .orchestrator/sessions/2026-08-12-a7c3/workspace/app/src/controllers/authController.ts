import { Request, Response, NextFunction } from 'express';
import * as authService from '../services/authService';
import { z } from 'zod';

const registerSchema = z.object({
  email: z.string().email(),
  password: z.string().min(6),
  name: z.string().optional(),
});

const loginSchema = z.object({
  email: z.string().email(),
  password: z.string(),
});

export const register = async (req: Request, res: Response, next: NextFunction) => {
  try {
    const validatedData = registerSchema.parse(req.body);
    const result = await authService.registerUser(validatedData);
    res.status(201).json({ success: true, data: result, message: 'User registered successfully' });
  } catch (err: any) {
    if (err instanceof z.ZodError) {
      res.status(400).json({ success: false, message: 'Validation error', errors: (err as any).errors });
    } else {
      next(err);
    }
  }
};

export const login = async (req: Request, res: Response, next: NextFunction) => {
  try {
    const validatedData = loginSchema.parse(req.body);
    const result = await authService.loginUser(validatedData);
    res.status(200).json({ success: true, data: result, message: 'Login successful' });
  } catch (err: any) {
    if (err instanceof z.ZodError) {
      res.status(400).json({ success: false, message: 'Validation error', errors: (err as any).errors });
    } else {
      next(err);
    }
  }
};

export const getMe = async (req: Request, res: Response, next: NextFunction) => {
  try {
    const userId = (req as any).user.userId;
    const user = await authService.getUserById(userId);
    res.status(200).json({ success: true, data: user });
  } catch (err) {
    next(err);
  }
};
