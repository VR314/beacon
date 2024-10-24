<h1 align="center">
    Beacon
</h1>

<p align="center">
    A plug-and-play commanding and telemetry server.
</p>

`beacon` is a project meant to create a flexible commanding and telemetry server, meant to easily integrate into projects without introducing additional constraints. 

The commanding server is meant to accepts packets using the Beacon Packet Protocol (defined later), to run registered command handler functions.

The telemetry server is meant to periodically flush telemetry data in the BPP, with flexiblity in packetization and rate of flushing.

The current implementation connects the two servers over IPC, and then performs the commanding and telemetry logic.

## Organization
```
├── beacon
├── client
├── server
└── beacon_tests
```

`beacon` holds the core code, and is the crate that is meant to be used when integrating `beacon` into a codebase.

`client` is an example client binary, and `server` is a corresponding server binary.

`beacon_tests` holds integration tests, where a client and server are spooled up and talk to each other during the tests.

