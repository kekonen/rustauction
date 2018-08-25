FROM node:latest


WORKDIR /home/node

RUN chown -Rf node:node .

COPY --chown=node:node ./client/package.json .

#USER node
RUN npm install && npm cache clean --force
RUN npm install -g @vue/cli
