FROM node:alpine as builder
ARG REACT_APP_URL=localhost:8080
ARG REACT_APP_DOCKER=true

WORKDIR /app
COPY website .
RUN npm i && npm run build

FROM nginx:alpine
ARG API_PORT=8080

RUN apk add gettext

WORKDIR /app
COPY --from=builder /app/build /var/www/kek

COPY docker/website/site.conf .
RUN envsubst '$API_PORT' < site.conf > /etc/nginx/conf.d/default.conf

VOLUME ["/var/log/websites"]

CMD ["nginx", "-g", "daemon off;"]
