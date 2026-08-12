import request from 'supertest';
import app from '../src/app';
import { prisma } from '../src/config/database';

let token: string;
let taskId: string;

describe('Task Endpoints', () => {
  beforeAll(async () => {
    await prisma.task.deleteMany({});
    await prisma.user.deleteMany({});

    // Create user and get token
    const res = await request(app)
      .post('/api/auth/register')
      .send({
        email: 'taskuser@example.com',
        password: 'password123',
        name: 'Task User'
      });
    token = res.body.data.token;
  });

  afterAll(async () => {
    await prisma.$disconnect();
  });

  it('should create a new task', async () => {
    const res = await request(app)
      .post('/api/tasks')
      .set('Authorization', `Bearer ${token}`)
      .send({
        title: 'New Task',
        description: 'Test task'
      });
    expect(res.statusCode).toEqual(201);
    expect(res.body.success).toBe(true);
    expect(res.body.data).toHaveProperty('id');
    taskId = res.body.data.id;
  });

  it('should fetch tasks', async () => {
    const res = await request(app)
      .get('/api/tasks')
      .set('Authorization', `Bearer ${token}`);
    expect(res.statusCode).toEqual(200);
    expect(res.body.data.length).toBeGreaterThan(0);
  });

  it('should update a task', async () => {
    const res = await request(app)
      .patch(`/api/tasks/${taskId}`)
      .set('Authorization', `Bearer ${token}`)
      .send({
        status: 'DONE'
      });
    expect(res.statusCode).toEqual(200);
    expect(res.body.data.status).toEqual('DONE');
  });

  it('should delete a task', async () => {
    const res = await request(app)
      .delete(`/api/tasks/${taskId}`)
      .set('Authorization', `Bearer ${token}`);
    expect(res.statusCode).toEqual(204);
  });
});
