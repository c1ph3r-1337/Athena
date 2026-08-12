import bcrypt from 'bcryptjs';
import { CustomError } from '../utils/customError.js';
import { signAccessToken } from '../utils/jwt.util.js';

export interface UserRecord {
  id: string;
  email: string;
  passwordHash: string;
  createdAt?: Date;
}

export interface UserRepository {
  findByEmail(email: string): Promise<UserRecord | null>;
  create(input: { email: string; passwordHash: string }): Promise<UserRecord>;
}

export interface PublicUser {
  id: string;
  email: string;
  createdAt?: Date;
}

export interface AuthResult {
  user: PublicUser;
  accessToken: string;
}

const BCRYPT_ROUNDS = 12;
// Equalizes the "unknown user" login path enough to avoid revealing account existence by timing.
const DUMMY_HASH = '$2a$12$7CXUc5YFaqNtQ6iE.2xNIubGawDE.rSBg1LqCVcb3qawq1nUMTVw2';

const publicUser = ({ id, email, createdAt }: UserRecord): PublicUser => ({ id, email, createdAt });

export class AuthService {
  constructor(private readonly users: UserRepository) {}

  async register(email: string, password: string): Promise<AuthResult> {
    const normalizedEmail = email.trim().toLowerCase();
    if (await this.users.findByEmail(normalizedEmail)) {
      throw new CustomError(409, 'An account with this email already exists.', 'EMAIL_ALREADY_REGISTERED');
    }
    const passwordHash = await bcrypt.hash(password, BCRYPT_ROUNDS);
    const user = await this.users.create({ email: normalizedEmail, passwordHash });
    return { user: publicUser(user), accessToken: signAccessToken({ sub: user.id, email: user.email }) };
  }

  async login(email: string, password: string): Promise<AuthResult> {
    const user = await this.users.findByEmail(email.trim().toLowerCase());
    const passwordMatches = await bcrypt.compare(password, user?.passwordHash ?? DUMMY_HASH);
    if (!user || !passwordMatches) {
      throw new CustomError(401, 'Invalid email or password.', 'INVALID_CREDENTIALS');
    }
    return { user: publicUser(user), accessToken: signAccessToken({ sub: user.id, email: user.email }) };
  }
}
