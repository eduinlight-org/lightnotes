-include .env

export API_BASE_URL

landing-dev:
	dx serve --package landing --platform web --port $(LANDING_PORT)

landing-build:
	dx build --package landing --platform web --fullstack true --release

app-web-dev:
	dx serve --package web --platform web --port $(APP_WEB_PORT)

app-android-dev:
	dx serve --package light-notes-mobile --platform android

app-ios-dev:
	dx serve --package light-notes-mobile --platform ios

app-desktop-dev:
	dx serve --package desktop --platform desktop

app-web-build:
	dx build --package web --platform web --fullstack true --release

app-android-build:
	dx build --package light-notes-mobile --platform android --release

app-ios-build:
	dx build --package light-notes-mobile --platform ios --release

app-desktop-build:
	dx build --package desktop --platform desktop --release

landing-bundle:
	dx bundle --package landing --platform web --fullstack true --release

app-web-bundle:
	dx bundle --package web --platform web --fullstack true --release

app-macos-bundle:
	./scripts/macos-bundle.sh

app-windows-bundle:
	pwsh -NoProfile -ExecutionPolicy Bypass -File scripts/windows-bundle.ps1

app-linux-bundle:
	./scripts/linux-bundle.sh

app-android-bundle:
	./scripts/android-bundle.sh

app-ios-bundle:
	dx bundle --package light-notes-mobile --platform ios --release --package-types ipa

app-bundle-all: app-web-bundle app-macos-bundle app-windows-bundle app-linux-bundle app-android-bundle app-ios-bundle

api-dev:
	cargo run -p api

api-build:
	cargo build -p api --release

docker-up:
	docker compose up -d

docker-down:
	docker compose down
