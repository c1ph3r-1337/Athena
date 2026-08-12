import { prisma } from '../config/database';
import bcrypt from 'bcryptjs';
import jwt from 'jsonwebtoken';

const JWT_SECRET = process.env.JWT_SECRET || 'supersecretkey123';
const JWT_EXPIRES_IN = '1d';

export const registerUser = async (data: any) => {
  const existingUser = await prisma.user.findUnique({ where: { email: data.email } });
  if (existingUser) {
    const error: any = new Error('Email already in use');
    error.status = 409;
    throw error;
  }

  const salt = await bcrypt.genSalt(10);
  const passwordHash = await bcrypt.hash(data.password, salt);

  const user = await prisma.user.create({
    data: {
      email: data.email,
      passwordHash,
      name: data.name,
    },
  });

  const token = jwt.sign({ userId: user.id, role: user.role }, JWT_SECRET, {
    expiresIn: JWT_EXPIRES_IN,
  });

  return { user: { id: user.id, email: user.email, name: user.name, role: user.role }, token };
};

export const loginUser = async (data: any) => {
  const user = await prisma.user.findUnique({ where: { email: data.email } });
  if (!user) {
    const error: any = new Error('Invalid email or password');
    error.status = 401;
    throw error;
  }

  const isMatch = await bcrypt.compare(data.password, user.passwordHash);
  if (!isMatch) {
    const error: any = new Error('Invalid email or password');
    error.status = 401;
    throw error;
  }

  const token = jwt.sign({ userId: user.id, role: user.role }, JWT_SECRET, {
    expiresIn: JWT_EXPIRES_IN,
  });

  return { user: { id: user.id, email: user.email, name: user.name, role: user.role }, token };
};

export const getUserById = async (id: string) => {
  const user = await prisma.user.findUnique({ where: { id } });
  if (!user) {
    const error: any = new Error('User not found');
    error.status = 404;
    throw error;
  }
  return { id: user.id, email: user.email, name: user.name, role: user.role };
};
