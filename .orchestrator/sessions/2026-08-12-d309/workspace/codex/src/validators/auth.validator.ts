import { z } from 'zod';

const email = z.string().trim().email().max(254).transform((value) => value.toLowerCase());
const password = z.string().min(12, 'Password must contain at least 12 characters.').max(128);

export const registerSchema = z.object({ email, password });
export const loginSchema = z.object({ email, password: z.string().min(1).max(128) });
