# rsky-pds: Personal Data Server (PDS)

A Rust implementation of an atproto PDS.

## What is a Personal Data Server?
As defined by the [AT Protocol documentation](https://atproto.com/guides/glossary#pds-personal-data-server), a Personal
Data Server (PDS), is a server that hosts a user. It is responsible for hosting the user's 
[data repository](https://atproto.com/guides/glossary#pds-personal-data-server), and signing keys. It also has the
ability to assign handles (e.g. user.rsky.social), and DIDs to users. PDSes can host multiple users.

A PDS communicates with [AppViews](https://atproto.com/guides/glossary#pds-personal-data-server)
to run applications. A PDS doesn't typically run any applications itself, though it will have general account management 
interfaces such as the OAuth login screen. PDSes actively sync their data repositories with 
[Relays](https://atproto.com/guides/glossary#relay).


## License
rsky is released under the [Apache License 2.0](../LICENSE).