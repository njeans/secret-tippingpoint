FROM node:16

RUN yarn global add @vue/cli

WORKDIR /usr/src/medtee

COPY . .

WORKDIR /usr/src/medtee

RUN yarn && yarn add @stakeordie/griptape.js
