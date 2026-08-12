FROM node:18-alpine

WORKDIR /app

COPY package*.json ./
COPY prisma ./prisma/

RUN npm install

COPY . .

RUN npm run build
# Normally we would run npx prisma generate here or in postinstall

EXPOSE 3000

CMD ["npm", "start"]
