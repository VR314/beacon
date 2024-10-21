<h1 align="center">
    Beacon
</h1>

<p align="center">
    A plug-and-play commanding and telemetry server.
</p>

`beacon` is a project meant to create a flexible commanding and telemetry server, meant to easily integrate into projects without introducing additional constraints. 

The commanding server is meant to accepts packets using the Beacon Packet Protocol (defined later), to run registered command handler functions.

The telemetry server is meant to periodically flush telemetry data in the BPP, with flexiblity in packetization and rate of flushing.
