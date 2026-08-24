---
name: youtube-cli
description: CLI access to YouTube video search, video and channel metadata, channel and playlist videos, transcripts, and comments. Use when the request needs information from YouTube content.
---

`youtube-cli` is installed and available on `PATH`.

In Antigravity, if the plugin `bin/` directory is not on `PATH`, execute this plugin's `bin/youtube-cli` shim directly.

- Run `youtube-cli --help` to discover resources.
- Run `youtube-cli <resource> --help` and `youtube-cli <resource> <command> --help` to discover commands and options.

Transcript and comment commands save their complete results to the OS temporary directory and return the file path.
