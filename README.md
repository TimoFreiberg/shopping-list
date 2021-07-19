# shopping-list

Shopping list.

Manually invited people share the same shopping list.
This is meant for people that don't want to use one of the hundreds of free cloud todo list apps for some reason.
Maybe because they feel spied on, who knows.

The `ROCKET_SECRET_KEY` in `.env` is only used locally, don't mind it.

## How to setup a fly.io app

First off, you're gonna need to enter credit card info.
According to one of the founders, this is the best way to fend off a flood of crypto-miners, which I completely understand ([source](https://twitter.com/jeromegn/status/1388823002826121216)).
The [free tier](https://fly.io/docs/about/pricing/) seems fine for this kind of usage.

1. Install flyctl: https://fly.io/docs/getting-started/installing-flyctl/, https://aur.archlinux.org/packages/flyctl-bin/
1. `fly auth signup`/`fly auth login` (most installations should symlink `fly` to `flyctl`)
1. `fly launch --name shopping-list`
1. `fly postgres create`
1. `fly postgres attach --postgres-app POSTGRES-NAME` (this sets the `DATABASE_URL` secret)
1. `fly secrets set GOOGLE_CLIENT_ID=YOUR_GOOGLE_OAUTH_CLIENT_ID`
1. `fly secrets set GOOGLE_CLIENT_SECRET=YOUR_GOOGLE_OAUTH_CLIENT_SECRET`
1. `fly secrets set ROCKET_SECRET_KEY=(openssl rand -base64 32)`
1. `fly secrets set ALLOWED_EMAILS=you@example.com,your_significant_other@example.com` (insert your email addresses here)
1. `fly secrets set OAUTH_REDIRECT_URL=https://your-app-name.fly.dev`

If you can run Docker images and get access to a Postgres anywhere else, that'll work as well.

## Google OAuth Client

Set up a google oauth client at https://console.cloud.google.com/apis/credentials.
This app needs the `userinfo.email` scope.
Add the URL of your fly.io app to `Authorised redirect URIs`.

You'll need the Client ID and Client secret both for configuring prod and for testing locally.

## Contributing/Local Setup

For frontend:

1. Have `npm` installed (probably https://github.com/nvm-sh/nvm#installing-and-updating ?)
1. In the frontend directory: run `npm start`

For backend:

1. Have `cargo` installed (https://rustup.rs/)
1. Set `GOOGLE_CLIENT_ID` and `GOOGLE_CLIENT_SECRET`
1. Run `cargo run --release` or `cargo test`

Note: Only the REST API will work

For both:

1. In the frontend directory: run `npm run build`
1. Set `GOOGLE_CLIENT_ID` and `GOOGLE_CLIENT_SECRET`
1. Run `cargo run --release` or `cargo test`
1. Go to `localhost:8000`
