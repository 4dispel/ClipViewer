# ClipViewer
this is a twitch bot that connects to a chat and then provides commands to manage a queue for twitch clips

## about
the bot uses twitch-irc to connect to a twitch chat and uses thirtyfour to connect to a chromedriver and display twitch clips embeds


## setup
provide your own login_name, user access token with the scopes below, as well as stream to connect to, and client id in the creds.rs file
```
chat:edit
user:read:chat
user:bot
chat:read
```
you also need chromedriver running on port 9515 for the clips to be played

to use in obs add a window capture that only allows exactly the same name as the window

### run bot
```
cargo run
```
### compile bot without running
```
cargo build
```
you can then find the bot at /target/debug/clip_viewer.exe
name might be different if not on Windows
this is a debug mode build, to compile release mode add --release flag
## commands

```
everyone:

!cq <link> -> queue clip (queue up to 10 clips after current)
!cs -> show queue

Mods only:

!cr <number> -> go forward <number> clips and remove then go back to current
!cc -> clear queue

Broadcaster only:

!cp -> play current clip in queue

!cn -> go to next clip and play it
!cn <number> -> go <number> clips forward and play (max 15)

!cb -> got to last clip and play it
!cb <number> -> go back <number> clips and play (max 5)
```