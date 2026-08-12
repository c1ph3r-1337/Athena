import app from './app';
import { prisma, disconnectDatabase } from './config/database';

const PORT = process.env.PORT || 3000;

const startServer = async () => {
  try {
    // Check DB connection
    await prisma.$connect();
    console.log('Connected to database successfully');

    const server = app.listen(PORT, () => {
      console.log(`Server running on port ${PORT}`);
    });

    // Graceful shutdown
    const shutdown = async () => {
      console.log('Shutting down server...');
      server.close(async () => {
        await disconnectDatabase();
        console.log('Database disconnected');
        process.exit(0);
      });
    };

    process.on('SIGINT', shutdown);
    process.on('SIGTERM', shutdown);
  } catch (error) {
    console.error('Failed to start server:', error);
    process.exit(1);
  }
};

startServer();
