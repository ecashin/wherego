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

