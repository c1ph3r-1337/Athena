import request from 'supertest';
import app from '../src/app';

describe('Error Handling', () => {
  it('should return 404 for unknown route', async () => {
    const res = await request(app).get('/api/unknown-route-12345');
    expect(res.status).toBe(404);
  });
});
