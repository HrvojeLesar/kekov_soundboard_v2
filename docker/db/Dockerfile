FROM postgres:alpine
ARG PG_USER=kek
ARG PG_USER_PASSWORD=kek

RUN apk add gettext

WORKDIR /app
COPY docker/db/init.sql . 
RUN envsubst < init.sql > /docker-entrypoint-initdb.d/init.sql

CMD ["postgres"]
