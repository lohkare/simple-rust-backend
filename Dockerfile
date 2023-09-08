FROM rust

COPY target/debug/discord-bot /bin/discord-bot

CMD ["/bin/discord-bot"]