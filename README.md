# Wherego

Wherego is an insecure web platform
for helping people negotiate a shared travel destination.

## Insecure

Users do not authenticate but simply edit their username
to establish identity.
There is no special support for TLS.
Content is not filtered for spam, hate speech, etc.

The envisioned use case is running on a Raspberry Pi
in a home network with only trusted users on the network.

## Deploying

To build the browser part,
use [trunk](https://trunkrs.dev/).

    cd frontend
    trunk build

The server requires a [PostgreSQL](https://www.postgresql.org/)
database with a Postgres user who can create and update tables
in their user database.
The username and password are provided as command-line arguments to the server,
as shown below.

To build and run the server part,
use [cargo](https://doc.rust-lang.org/cargo/).

    cd backend
    # WARNING: WIPES OUT ANY EXISTING WHEREGO DATA
    cargo run -- --reinitialize-database myuser mypassword

On subsequent runs, the database can be preserved
by omitting the `--reinitialize-database` option.
In the example below, an IP is specified for listening.

    cd backend
    cargo run -- --listen-ip 192.168.1.111 myuser mypassword

There is usage help.

    cargo run -- --help

## Usage

Destinations are shared by all participants.
Scores are associated with a participant.
Participants must edit the username field before changing scores.

To "negotiate", one checks checkboxes near the top of the page
to select participants whose scores will be taken into account.
On hitting the "negotiate" button,
the preferences will be combined (using a Borda count),
and the most commonly preferred destinations will be displayed
at the top, with increasingly dispicable destinations following.

