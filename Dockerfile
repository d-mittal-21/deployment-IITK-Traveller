FROM node:alpine AS build

WORKDIR /usr/app

COPY . /usr/app

RUN npm ci

RUN npm run build

FROM nginx

COPY --from=build /usr/app/build /usr/share/nginx/html