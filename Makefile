-include .env

landing-dev:
	dx serve --package landing --platform web --port $(LANDING_PORT)

landing-build:
	dx build --package landing --platform web --fullstack true

app-web-dev:
	dx serve --package web --platform web --port $(APP_WEB_PORT)

app-android-dev:
	dx serve --package mobile --platform android

app-ios-dev:
	dx serve --package mobile --platform ios

app-desktop-dev:
	dx serve --package desktop --platform desktop
