FROM node:16.13.0
WORKDIR /srv

# Install CLI tools or other OS library dependencies
RUN npm install -g pnpm
