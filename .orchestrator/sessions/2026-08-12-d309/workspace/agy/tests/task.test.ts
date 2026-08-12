import request from 'supertest';
import app from '../src/app';
import prisma from '../src/config/database';

beforeAll(async () => {
  // Clear the task table before tests (if using a test db)
});

afterAll(async () => {
  await prisma.$disconnect();
});

describe('Task API', () => {
  let taskId: string;

  it('should create a new task', async () => {
    const res = await request(app)
      .post('/api/tasks')
      .send({ title: 'Test Task', description: 'Testing the API' });
      
    expect(res.status).toBe(201);
    expect(res.body).toHaveProperty('id');
    taskId = res.body.id;
  });

  it('should fetch all tasks', async () => {
    const res = await request(app).get('/api/tasks');
    expect(res.status).toBe(200);
    expect(res.body.data).toBeInstanceOf(Array);
  });

  it('should fetch a single task', async () => {
    const res = await request(app).get(`/api/tasks/${taskId}`);
    expect(res.status).toBe(200);
    expect(res.body.title).toBe('Test Task');
  });

  it('should update a task', async () => {
    const res = await request(app)
      .put(`/api/tasks/${taskId}`)
      .send({ completed: true });
      
    expect(res.status).toBe(200);
    expect(res.body.completed).toBe(true);
  });

  it('should delete a task', async () => {
    const res = await request(app).delete(`/api/tasks/${taskId}`);
    expect(res.status).toBe(204);
  });
});
