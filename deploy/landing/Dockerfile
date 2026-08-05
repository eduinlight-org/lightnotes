FROM nginx:1.27-alpine

COPY deploy/landing/nginx.conf /etc/nginx/conf.d/default.conf
COPY target/dx/landing/release/web/public /usr/share/nginx/html

EXPOSE 80
