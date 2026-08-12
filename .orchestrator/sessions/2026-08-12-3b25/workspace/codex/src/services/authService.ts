import bcrypt from 'bcryptjs';
import crypto from 'node:crypto';
import jwt, { JwtPayload, SignOptions } from 'jsonwebtoken';

export type Role = 'user' | 'admin';

export interface AuthUser {
  id: string;
  email: string;
  passwordHash: string;
  role: Role;
  createdAt?: Date;
}

/** The data-access contract keeps authentication independent of the ORM in use. */
export interface UserRepository {
  findByEmail(email: string): Promise<AuthUser | null>;
  findById(id: string): Promise<AuthUser | null>;
  create(input: Pick<AuthUser, 'email' | 'passwordHash' | 'role'>): Promise<AuthUser>;
}

export interface PublicUser {
  id: string;
  email: string;
  role: Role;
}

export interface AuthTokens {
  accessToken: string;
  refreshToken: string;
  expiresIn: number;
}

export class AuthError extends Error {
  constructor(public readonly statusCode: number, message: string) {
    super(message);
    this.name = 'AuthError';
  }
}

interface TokenClaims extends JwtPayload {
  sub: string;
  email: string;
  role: Role;
  tokenType: 'access' | 'refresh';
}

export interface AuthServiceOptions {
  jwtSecret?: string;
  jwtRefreshSecret?: string;
  accessTokenTtl?: string;
  refreshTokenTtl?: string;
  bcryptRounds?: number;
}

export class AuthService {
  private readonly jwtSecret: string;
  private readonly refreshSecret: string;
  private readonly accessTokenTtl: string;
  private readonly refreshTokenTtl: string;
  private readonly bcryptRounds: number;

  constructor(private readonly users: UserRepository, options: AuthServiceOptions = {}) {
    this.jwtSecret = options.jwtSecret ?? process.env.JWT_SECRET ?? '';
    this.refreshSecret = options.jwtRefreshSecret ?? process.env.JWT_REFRESH_SECRET ?? '';
    this.accessTokenTtl = options.accessTokenTtl ?? process.env.JWT_ACCESS_EXPIRES_IN ?? '15m';
    this.refreshTokenTtl = options.refreshTokenTtl ?? process.env.JWT_REFRESH_EXPIRES_IN ?? '7d';
    this.bcryptRounds = options.bcryptRounds ?? Number(process.env.BCRYPT_ROUNDS ?? 12);

    if (!this.jwtSecret || !this.refreshSecret) {
      throw new Error('JWT_SECRET and JWT_REFRESH_SECRET must be configured');
    }
    if (!Number.isInteger(this.bcryptRounds) || this.bcryptRounds < 10 || this.bcryptRounds > 15) {
      throw new Error('BCRYPT_ROUNDS must be an integer between 10 and 15');
    }
  }

  async register(email: string, password: string, role: Role = 'user'): Promise<{ user: PublicUser; tokens: AuthTokens }> {
    const normalizedEmail = this.normalizeEmail(email);
    this.validatePassword(password);
    if (role !== 'user' && role !== 'admin') throw new AuthError(400, 'Invalid role');
    if (await this.users.findByEmail(normalizedEmail)) throw new AuthError(409, 'Email is already registered');

    const passwordHash = await bcrypt.hash(password, this.bcryptRounds);
    const user = await this.users.create({ email: normalizedEmail, passwordHash, role });
    return { user: this.toPublicUser(user), tokens: this.issueTokens(user) };
  }

  async login(email: string, password: string): Promise<{ user: PublicUser; tokens: AuthTokens }> {
    const user = await this.users.findByEmail(this.normalizeEmail(email));
    // Always run bcrypt to make unknown-user and bad-password responses comparable in timing.
    const passwordMatches = await bcrypt.compare(password, user?.passwordHash ?? '$2a$12$invalidinvalidinvalidinvalidinvalidinvalidinvalidinvalidinva');
    if (!user || !passwordMatches) throw new AuthError(401, 'Invalid email or password');
    return { user: this.toPublicUser(user), tokens: this.issueTokens(user) };
  }

  async refresh(refreshToken: string): Promise<AuthTokens> {
    const claims = this.verifyToken(refreshToken, this.refreshSecret, 'refresh');
    const user = await this.users.findById(claims.sub);
    if (!user) throw new AuthError(401, 'Refresh token is no longer valid');
    return this.issueTokens(user);
  }

  verifyAccessToken(token: string): TokenClaims {
    return this.verifyToken(token, this.jwtSecret, 'access');
  }

  private issueTokens(user: AuthUser): AuthTokens {
    const baseClaims = { email: user.email, role: user.role };
    const accessToken = jwt.sign({ ...baseClaims, tokenType: 'access' }, this.jwtSecret, {
      subject: user.id, expiresIn: this.accessTokenTtl as SignOptions['expiresIn'], issuer: 'rest-api', audience: 'rest-api-client', jwtid: crypto.randomUUID(),
    });
    const refreshToken = jwt.sign({ ...baseClaims, tokenType: 'refresh' }, this.refreshSecret, {
      subject: user.id, expiresIn: this.refreshTokenTtl as SignOptions['expiresIn'], issuer: 'rest-api', audience: 'rest-api-client', jwtid: crypto.randomUUID(),
    });
    return { accessToken, refreshToken, expiresIn: this.durationToSeconds(this.accessTokenTtl) };
  }

  private verifyToken(token: string, secret: string, tokenType: TokenClaims['tokenType']): TokenClaims {
    try {
      const decoded = jwt.verify(token, secret, { issuer: 'rest-api', audience: 'rest-api-client' });
      if (typeof decoded === 'string' || decoded.tokenType !== tokenType || !decoded.sub || !decoded.email || !decoded.role) throw new Error('Invalid claims');
      return decoded as TokenClaims;
    } catch {
      throw new AuthError(401, 'Invalid or expired token');
    }
  }

  private normalizeEmail(email: string): string {
    const normalized = typeof email === 'string' ? email.trim().toLowerCase() : '';
    if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(normalized) || normalized.length > 254) throw new AuthError(400, 'A valid email is required');
    return normalized;
  }

  private validatePassword(password: string): void {
    if (typeof password !== 'string' || password.length < 12 || password.length > 128) {
      throw new AuthError(400, 'Password must be between 12 and 128 characters');
    }
  }

  private toPublicUser(user: AuthUser): PublicUser {
    return { id: user.id, email: user.email, role: user.role };
  }

  private durationToSeconds(ttl: string): number {
    const match = /^(\d+)\s*([smhd])$/.exec(ttl);
    if (!match) return 900;
    return Number(match[1]) * ({ s: 1, m: 60, h: 3600, d: 86400 } as Record<string, number>)[match[2]];
  }
}
