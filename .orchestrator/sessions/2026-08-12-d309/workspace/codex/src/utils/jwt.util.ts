import jwt, { JwtPayload, SignOptions } from 'jsonwebtoken';
import { CustomError } from './customError.js';

export interface AccessTokenPayload extends JwtPayload {
  sub: string;
  email: string;
}

const getJwtSecret = (): string => {
  const secret = process.env.JWT_SECRET;
  if (!secret || secret.length < 32) {
    throw new CustomError(500, 'JWT_SECRET must be set to at least 32 characters.', 'JWT_CONFIGURATION_ERROR');
  }
  return secret;
};

const getExpiration = (): NonNullable<SignOptions['expiresIn']> =>
  (process.env.JWT_EXPIRES_IN as NonNullable<SignOptions['expiresIn']>) || '15m';

export const signAccessToken = (payload: Pick<AccessTokenPayload, 'sub' | 'email'>): string =>
  jwt.sign(payload, getJwtSecret(), {
    algorithm: 'HS256',
    expiresIn: getExpiration(),
    issuer: process.env.JWT_ISSUER || 'rest-api',
    audience: process.env.JWT_AUDIENCE || 'rest-api-clients'
  });

export const verifyAccessToken = (token: string): AccessTokenPayload => {
  try {
    const decoded = jwt.verify(token, getJwtSecret(), {
      algorithms: ['HS256'],
      issuer: process.env.JWT_ISSUER || 'rest-api',
      audience: process.env.JWT_AUDIENCE || 'rest-api-clients'
    });
    if (typeof decoded === 'string' || !decoded.sub || !decoded.email) throw new Error('Invalid claims');
    return decoded as AccessTokenPayload;
  } catch {
    throw new CustomError(401, 'Invalid or expired access token.', 'INVALID_TOKEN');
  }
};
