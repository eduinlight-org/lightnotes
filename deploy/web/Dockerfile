FROM nginx:1.27-alpine

COPY deploy/spa.conf /etc/nginx/conf.d/default.conf
COPY target/dx/web/release/web/public /usr/share/nginx/html

EXPOSE 80
