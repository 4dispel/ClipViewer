use ::twitch_irc::message::ServerMessage;
use serde::Deserialize;
use std::{thread, time::Duration};
use thirtyfour::prelude::*;
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::message::PrivmsgMessage;
use twitch_irc::transport::tcp::TCPTransport;
use twitch_irc::ClientConfig;
use twitch_irc::SecureTCPTransport;
use twitch_irc::TwitchIRCClient;
use url::Url;
mod creds;
#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let config = ClientConfig::new_simple(StaticLoginCredentials::new(
        creds::LOGIN_NAME.to_owned(),
        Some(creds::OAUTH_TOKEN.to_owned()),
    ));
    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);
    let mut slowmodetime: Duration = Duration::from_secs(1);
    let client2 = client.clone();
    let join_handle = tokio::spawn(async move {
        let mut moderator = creds::LOGIN_NAME == creds::STREAM_NAME;
        let mut queue = ClipQueue::new();
        while let Some(message) = incoming_messages.recv().await {
            match message {
                ServerMessage::Privmsg(msg) => {
                    println!(
                        "(#{}) {}: {}",
                        msg.channel_login, msg.sender.name, msg.message_text
                    );
                    //remove the last 5 bytes twitch adds to duplicate messages
                    let unmangled_text: String = if msg.message_text.len() > 5 {
                        if &msg.message_text[msg.message_text.len() - 5..]
                            == std::str::from_utf8(&[0x20, 0xF3, 0xA0, 0x80, 0x80]).unwrap()
                        {
                            msg.message_text[..msg.message_text.len() - 4].to_owned()
                        } else {
                            msg.message_text.clone()
                        }
                    } else {
                        msg.message_text.clone()
                    };
                    match unmangled_text {
                        text if text.starts_with('!') => match text[1..].to_string() {
                            // clip request for deprecated autoplay version
                            // command if command.starts_with("cq") => {
                            //     if msg.sender.name == login_name2
                            //         && login_name2 != stream2
                            //         && !moderator
                            //     {
                            //         println!("sleeping");
                            //         thread::sleep(slowmodetime)
                            //     }
                            //     _clip_request(command, &client2, &oauth_token2, &client_id2, &msg)
                            //         .await;
                            //     if login_name2 != stream2 && !moderator {
                            //         println!("sleeping");
                            //         thread::sleep(slowmodetime)
                            //     }
                            // }
                            command if command.to_lowercase().starts_with("cq") => {
                                if msg.sender.name == creds::LOGIN_NAME
                                    && creds::LOGIN_NAME != creds::STREAM_NAME
                                    && !moderator
                                {
                                    println!("sleeping");
                                    thread::sleep(slowmodetime)
                                }
                                _enqueue_clip(
                                    &mut queue,
                                    command,
                                    &client2,
                                    &msg,
                                    &creds::OAUTH_TOKEN.to_string(),
                                    &creds::CLIENT_ID.to_string(),
                                )
                                .await;
                                if creds::LOGIN_NAME != creds::STREAM_NAME && !moderator {
                                    println!("sleeping");
                                    thread::sleep(slowmodetime)
                                }
                            }
                            command if command.to_lowercase().starts_with("cp") => {
                                if msg.sender.name == creds::LOGIN_NAME
                                    && creds::LOGIN_NAME != creds::STREAM_NAME
                                    && !moderator
                                {
                                    println!("sleeping");
                                    thread::sleep(slowmodetime)
                                }
                                _run_current(
                                    &mut queue,
                                    &msg,
                                    &client2,
                                    &creds::STREAM_NAME.to_string(),
                                )
                                .await;
                                if creds::LOGIN_NAME != creds::STREAM_NAME && !moderator {
                                    println!("sleeping");
                                    thread::sleep(slowmodetime)
                                }
                            }
                            command if command.to_lowercase().starts_with("cn ") => {
                                if msg.sender.name == creds::LOGIN_NAME
                                    && creds::LOGIN_NAME != creds::STREAM_NAME
                                    && !moderator
                                {
                                    println!("sleeping");
                                    thread::sleep(slowmodetime)
                                }
                                _run_next_n(
                                    &mut queue,
                                    &msg,
                                    &client2,
                                    &creds::STREAM_NAME.to_string(),
                                    command,
                                )
                                .await;
                                if creds::LOGIN_NAME != creds::STREAM_NAME && !moderator {
                                    println!("sleeping");
                                    thread::sleep(slowmodetime)
                                }
                            }
                            command if command.to_lowercase().starts_with("cn") => {
                                if msg.sender.name == creds::LOGIN_NAME
                                    && creds::LOGIN_NAME != creds::STREAM_NAME
                                    && !moderator
                                {
                                    println!("sleeping");
                                    thread::sleep(slowmodetime)
                                }
                                _run_next(
                                    &mut queue,
                                    &msg,
                                    &client2,
                                    &creds::STREAM_NAME.to_string(),
                                )
                                .await;
                                if creds::LOGIN_NAME != creds::STREAM_NAME && !moderator {
                                    println!("sleeping");
                                    thread::sleep(slowmodetime)
                                }
                            }
                            command if command.to_lowercase().starts_with("cb ") => {
                                if msg.sender.name == creds::LOGIN_NAME
                                    && creds::LOGIN_NAME != creds::STREAM_NAME
                                    && !moderator
                                {
                                    println!("sleeping");
                                    thread::sleep(slowmodetime)
                                }
                                _run_previous_n(
                                    &mut queue,
                                    &msg,
                                    &client2,
                                    &creds::STREAM_NAME.to_string(),
                                    command,
                                )
                                .await;
                                if creds::LOGIN_NAME != creds::STREAM_NAME && !moderator {
                                    println!("sleeping");
                                    thread::sleep(slowmodetime)
                                }
                            }
                            command if command.to_lowercase().starts_with("cb") => {
                                if msg.sender.name == creds::LOGIN_NAME
                                    && creds::LOGIN_NAME != creds::STREAM_NAME
                                    && !moderator
                                {
                                    println!("sleeping");
                                    thread::sleep(slowmodetime)
                                }
                                _run_previous(
                                    &mut queue,
                                    &msg,
                                    &client2,
                                    &creds::STREAM_NAME.to_string(),
                                )
                                .await;
                                if creds::LOGIN_NAME != creds::STREAM_NAME && !moderator {
                                    println!("sleeping");
                                    thread::sleep(slowmodetime)
                                }
                            }
                            command if command.to_lowercase().starts_with("cc") => {
                                if msg.sender.name == creds::LOGIN_NAME
                                    && creds::LOGIN_NAME != creds::STREAM_NAME
                                    && !moderator
                                {
                                    println!("sleeping");
                                    thread::sleep(slowmodetime)
                                }
                                _clear_queue(&mut queue, &client2, &msg).await;
                                if creds::LOGIN_NAME != creds::STREAM_NAME && !moderator {
                                    println!("sleeping");
                                    thread::sleep(slowmodetime)
                                }
                            }
                            command if command.to_lowercase().starts_with("cs") => {
                                if msg.sender.name == creds::LOGIN_NAME
                                    && creds::LOGIN_NAME != creds::STREAM_NAME
                                    && !moderator
                                {
                                    println!("sleeping");
                                    thread::sleep(slowmodetime)
                                }
                                _show_queue(&mut queue, &client2, &msg).await;
                                if creds::LOGIN_NAME != creds::STREAM_NAME && !moderator {
                                    println!("sleeping");
                                    thread::sleep(slowmodetime)
                                }
                            }
                            command if command.to_lowercase().starts_with("cr ") => {
                                if msg.sender.name == creds::LOGIN_NAME
                                    && creds::LOGIN_NAME != creds::STREAM_NAME
                                    && !moderator
                                {
                                    println!("sleeping");
                                    thread::sleep(slowmodetime)
                                }
                                _remove_clip(&mut queue, &msg, &client2, command).await;
                                if creds::LOGIN_NAME != creds::STREAM_NAME && !moderator {
                                    println!("sleeping");
                                    thread::sleep(slowmodetime)
                                }
                            }
                            _ => {}
                        },
                        _ => {}
                    }
                }
                ServerMessage::Whisper(msg) => {
                    println!("(w) {}: {}", msg.sender.name, msg.message_text);
                }
                ServerMessage::RoomState(msg) if msg.slow_mode.is_some() => match msg.slow_mode {
                    Some(x) if x > Duration::ZERO => {
                        slowmodetime = x;
                        println!("slowmode:{:?}", x)
                    }
                    _ => {}
                },
                ServerMessage::UserState(msg) => {
                    //println!("checking badges");
                    for badge in msg.badges {
                        //println!("{}", badge.name);
                        if badge.name == "moderator" {
                            moderator = true;
                        }
                    }
                }
                ServerMessage::Join(msg) => {
                    println!("succesfully conected to {}", msg.channel_login)
                }
                _ => {}
            }
        }
    });
    client.join(creds::STREAM_NAME.to_owned()).unwrap();
    join_handle.await.unwrap();
    Ok(())
}

//deprecated autoplay clip
async fn _clip_request(
    msg: String,
    client: &TwitchIRCClient<TCPTransport<twitch_irc::transport::tcp::TLS>, StaticLoginCredentials>,
    auth: &String,
    client_id: &String,
    orginal_message: &PrivmsgMessage,
) {
    match _run_clip(msg[3..].to_string(), auth, client_id).await {
        Ok(_) => _reply_msg("clip finished playing".to_owned(), client, orginal_message).await,
        Err(_) => _reply_msg("couldn't queue clip".to_owned(), client, orginal_message).await,
    }
}

async fn _run_next(
    queue: &mut ClipQueue,
    orginal_message: &PrivmsgMessage,
    client: &TwitchIRCClient<TCPTransport<twitch_irc::transport::tcp::TLS>, StaticLoginCredentials>,
    channel: &String,
) {
    if !_msg_is_bc(orginal_message).await {
        _reply_msg(
            "need to be broadcaster to use".to_owned(),
            client,
            orginal_message,
        )
        .await;
        return;
    }
    match queue.advance(1) {
        Err(_) => _reply_msg("no clip next in queue".to_owned(), client, orginal_message).await,
        Ok(_) => _run_current(queue, orginal_message, client, channel).await,
    }
}

async fn _run_next_n(
    queue: &mut ClipQueue,
    orginal_message: &PrivmsgMessage,
    client: &TwitchIRCClient<TCPTransport<twitch_irc::transport::tcp::TLS>, StaticLoginCredentials>,
    channel: &String,
    text: String,
) {
    if !_msg_is_bc(orginal_message).await {
        _reply_msg(
            "need to be broadcaster to use".to_owned(),
            client,
            orginal_message,
        )
        .await;
        return;
    }
    let n: usize = match text[2..].trim().parse() {
        Ok(a) => a,
        Err(_) => {
            _reply_msg("no valid integer".to_owned(), client, orginal_message).await;
            return;
        }
    };
    // _reply_msg(format!("{}", n + 1), client, orginal_message).await;
    match queue.advance(n) {
        Err(_) => {
            _reply_msg(
                "no clip next at that position".to_owned(),
                client,
                orginal_message,
            )
            .await
        }
        Ok(_) => _run_current(queue, orginal_message, client, channel).await,
    }
}
async fn _run_previous(
    queue: &mut ClipQueue,
    orginal_message: &PrivmsgMessage,
    client: &TwitchIRCClient<TCPTransport<twitch_irc::transport::tcp::TLS>, StaticLoginCredentials>,
    channel: &String,
) {
    if !_msg_is_bc(orginal_message).await {
        _reply_msg(
            "need to be broadcaster to use".to_owned(),
            client,
            orginal_message,
        )
        .await;
        return;
    }
    match queue.rewind(1) {
        Err(_) => {
            _reply_msg(
                "no previous clip in queue".to_owned(),
                client,
                orginal_message,
            )
            .await
        }
        Ok(_) => _run_current(queue, orginal_message, client, channel).await,
    }
}

async fn _run_previous_n(
    queue: &mut ClipQueue,
    orginal_message: &PrivmsgMessage,
    client: &TwitchIRCClient<TCPTransport<twitch_irc::transport::tcp::TLS>, StaticLoginCredentials>,
    channel: &String,
    text: String,
) {
    if !_msg_is_bc(orginal_message).await {
        _reply_msg(
            "need to be broadcaster to use".to_owned(),
            client,
            orginal_message,
        )
        .await;
        return;
    }
    let n: usize = match text[2..].trim().parse() {
        Ok(a) => a,
        Err(_) => {
            _reply_msg("no valid integer".to_owned(), client, orginal_message).await;
            return;
        }
    };
    // _reply_msg(format!("{}", n + 1), client, orginal_message).await;
    match queue.rewind(n) {
        Err(_) => {
            _reply_msg(
                "no clip next at that position".to_owned(),
                client,
                orginal_message,
            )
            .await
        }
        Ok(_) => _run_current(queue, orginal_message, client, channel).await,
    }
}

async fn _remove_clip(
    queue: &mut ClipQueue,
    orginal_message: &PrivmsgMessage,
    client: &TwitchIRCClient<TCPTransport<twitch_irc::transport::tcp::TLS>, StaticLoginCredentials>,
    text: String,
) {
    if !_msg_is_mod(orginal_message).await {
        _reply_msg(
            "need to be at least moderator to use".to_owned(),
            client,
            orginal_message,
        )
        .await;
        return;
    }
    let i: i32 = match text[2..].trim().parse::<i32>() {
        Ok(a) => a + 5,
        Err(_) => {
            _reply_msg("no valid number".to_owned(), client, orginal_message).await;
            return;
        }
    };
    if i < 0 {
        _reply_msg(
            "clips aren't saved for that long".to_owned(),
            client,
            orginal_message,
        )
        .await;
        return;
    }
    match queue.remove(i as usize) {
        Ok(_) => _reply_msg("clip removed".to_owned(), client, orginal_message).await,
        Err(_) => {
            _reply_msg(
                "no clip at that point in queue".to_owned(),
                client,
                orginal_message,
            )
            .await
        }
    }
}

async fn _run_current(
    queue: &mut ClipQueue,
    orginal_message: &PrivmsgMessage,
    client: &TwitchIRCClient<TCPTransport<twitch_irc::transport::tcp::TLS>, StaticLoginCredentials>,
    channel: &String,
) {
    if !_msg_is_bc(orginal_message).await {
        _reply_msg(
            "need to be broadcaster to use".to_owned(),
            client,
            orginal_message,
        )
        .await;
        return;
    }
    if queue.remaining_clips > 0 {
        let clip = queue.current().unwrap();
        _reply_msg(
            format!("playing clip: {}", clip.title),
            client,
            orginal_message,
        )
        .await;
        match clip.run().await {
            Ok(_) => {
                println!("clip finished playing");
            }
            Err(_) => {
                println!("check if chromedriver is running");
                _send_msg(
                    "something went wrong playing the clip".to_owned(),
                    client,
                    channel,
                )
                .await
            }
        }
    } else {
        _reply_msg("no clip queued".to_owned(), client, orginal_message).await;
    }
}

async fn _reply_msg(
    msg: String,
    client: &TwitchIRCClient<TCPTransport<twitch_irc::transport::tcp::TLS>, StaticLoginCredentials>,
    orginal_message: &PrivmsgMessage,
) {
    if client
        .say_in_reply_to(orginal_message, msg.clone())
        .await
        .is_err()
    {
        println!("coudn't send reply \"{}\"", msg)
    }
}

async fn _send_msg(
    msg: String,
    client: &TwitchIRCClient<TCPTransport<twitch_irc::transport::tcp::TLS>, StaticLoginCredentials>,
    channel: &String,
) {
    if client.say(channel.to_string(), msg.clone()).await.is_err() {
        println!("coudn't send reply \"{}\"", msg)
    }
}

async fn _show_queue(
    queue: &mut ClipQueue,
    client: &TwitchIRCClient<TCPTransport<twitch_irc::transport::tcp::TLS>, StaticLoginCredentials>,
    orginal_message: &PrivmsgMessage,
) {
    let startindex = 5 - queue.previous_clips;
    let endindex = 4 + queue.remaining_clips;
    let mut sum_text = "".to_string();
    for i in startindex..=endindex {
        let clip = match queue.queue[i].clone() {
            Some(c) => c,
            None => {
                println!("attempt to access queue failed");
                return;
            }
        };
        // let j: i32 = i - 5;
        sum_text += format!("clip {}: {}  | ", i as i32 - 5, clip.title).as_str();
    }
    if sum_text.is_empty() {
        sum_text = "queue empty".to_string();
    }
    if sum_text.len() + 1 + orginal_message.sender.name.len() > 495 {
        _reply_msg(
            "queue is too long to display as twitch message".to_owned(),
            client,
            orginal_message,
        )
        .await;
        return;
    }
    _reply_msg(sum_text, client, orginal_message).await;
}

async fn _clear_queue(
    queue: &mut ClipQueue,
    client: &TwitchIRCClient<TCPTransport<twitch_irc::transport::tcp::TLS>, StaticLoginCredentials>,
    orginal_message: &PrivmsgMessage,
) {
    if !_msg_is_mod(orginal_message).await {
        _reply_msg(
            "need to be at least moderator to use".to_owned(),
            client,
            orginal_message,
        )
        .await;
        return;
    }
    queue.clear();
    _reply_msg("clip queue cleared".to_owned(), client, orginal_message).await;
}

async fn _msg_is_mod(msg: &PrivmsgMessage) -> bool {
    for badge in msg.badges.clone() {
        if badge.name == "moderator" || badge.name == "broadcaster" {
            return true;
        }
    }
    false
}

async fn _msg_is_bc(msg: &PrivmsgMessage) -> bool {
    for badge in msg.badges.clone() {
        if badge.name == "broadcaster" {
            return true;
        }
    }
    false
}

async fn _enqueue_clip(
    queue: &mut ClipQueue,
    text: String,
    client: &TwitchIRCClient<TCPTransport<twitch_irc::transport::tcp::TLS>, StaticLoginCredentials>,
    orginal_message: &PrivmsgMessage,
    auth: &String,
    client_id: &String,
) {
    match check_for_clip(&text[3..], auth, client_id).await {
        Some(res) => {
            let clip = ReceivedClipRequest {
                duration: res.0,
                title: res.2.clone(),
                id: res.1.clone(),
            };
            match queue.enqueue(clip) {
                Ok(a) => {
                    let reply = match queue.queue[a].clone() {
                        Some(c) => format!("clip: \"{}\" queued at position: {}", c.title, a - 5),
                        None => format!(
                            "clip: queued at position: {}, but couldn't find title",
                            a - 5
                        ),
                    };
                    _reply_msg(reply, client, orginal_message).await
                }
                Err(a) => {
                    _reply_msg(
                        format!("couldn't queue clip because: {}", a),
                        client,
                        orginal_message,
                    )
                    .await
                }
            }
        }
        None => _reply_msg("clip not found".to_owned(), client, orginal_message).await,
    }
}

//deprecated function for directly playing clips, functionality moved to _enqueue_clip and ReceivedClipRequest::run
//run clip from url directly
async fn _run_clip(url: String, auth: &String, client_id: &String) -> WebDriverResult<()> {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", caps).await?;
    let embed_url: String;
    let clip_duration: Duration;
    match check_for_clip(&url, auth, client_id).await {
        Some(a) => {
            clip_duration = Duration::from_secs_f32(a.0);
            embed_url = format!(
                "https://clips.twitch.tv/embed?clip={}&parent=www.mrredslime.com",
                a.1
            );
        }
        None => {
            println!("didn't find info");
            return Err(WebDriverError::CustomError("didn't find info".to_string()));
        }
    }

    driver.goto(embed_url).await?;

    match driver
        .find(By::Css(
            "button[data-a-target=\"content-classification-gate-overlay-start-watching-button\"]",
        ))
        .await
    {
        Ok(button) => button.click().await?,
        Err(_) => {
            match driver
                .find(By::Css(
                    "button[data-test-selector=\"player-overlay-play-button\"]",
                ))
                .await
            {
                Ok(button) => button.click().await?,
                Err(_) => {
                    println!("can't find the button")
                }
            }
        }
    }

    driver.fullscreen_window().await?;
    match driver
        .find(By::Css("button[aria-label=\"Unmute (m)\"]"))
        .await
    {
        Ok(button) => button.click().await?,
        Err(_) => println!("did not unmute, either something went wrong or it was already unmuted"),
    }

    thread::sleep(clip_duration);
    let mut done = false;
    while !done {
        done = driver
            .find(By::Css("button[aria-label=\"Replay (space/k)\"]"))
            .await
            .is_ok();
        if !done {
            println!("didn't find replay button");
            thread::sleep(Duration::from_secs(1));
        }
    }

    //thread::sleep(Duration::from_secs(30000));
    driver.quit().await?;

    Ok(())
}

struct ClipQueue {
    previous_clips: usize,
    remaining_clips: usize,
    queue: [Option<ReceivedClipRequest>; 21],
}

#[derive(Clone)]
struct ReceivedClipRequest {
    duration: f32,
    title: String,
    id: String,
}

impl ReceivedClipRequest {
    async fn run(self) -> WebDriverResult<()> {
        let caps = DesiredCapabilities::chrome();
        let driver = WebDriver::new("http://localhost:9515", caps).await?;
        let embed_url = format!(
            "https://clips.twitch.tv/embed?clip={}&parent=www.mrredslime.com",
            self.id
        );
        driver.goto(embed_url).await?;
        match driver
        .find(By::Css(
            "button[data-a-target=\"content-classification-gate-overlay-start-watching-button\"]",
        ))
        .await
    {
        Ok(button) => button.click().await?,
        Err(_) => {
            match driver
                .find(By::Css(
                    "button[data-test-selector=\"player-overlay-play-button\"]",
                ))
                .await
            {
                Ok(button) => button.click().await?,
                Err(_) => {
                    println!("can't find the button")
                }
            }
        }
    }

        driver.fullscreen_window().await?;
        match driver
            .find(By::Css("button[aria-label=\"Unmute (m)\"]"))
            .await
        {
            Ok(button) => button.click().await?,
            Err(_) => {
                println!("did not unmute, either something went wrong or it was already unmuted")
            }
        }
        let clip_duration = Duration::from_secs_f32(self.duration);
        thread::sleep(clip_duration);
        let mut done = false;
        while !done {
            done = driver
                .find(By::Css("button[aria-label=\"Replay (space/k)\"]"))
                .await
                .is_ok();
            if !done {
                println!("didn't find replay button");
                thread::sleep(Duration::from_secs(3));
            }
        }

        //thread::sleep(Duration::from_secs(30000));
        driver.quit().await?;
        Ok(())
    }
}

impl ClipQueue {
    fn new() -> ClipQueue {
        ClipQueue {
            previous_clips: 0,
            remaining_clips: 0,
            queue: Default::default(),
        }
    }
    fn advance(&mut self, n: usize) -> Result<(), ()> {
        if n > self.remaining_clips {
            return Err(());
        }
        for i in 0..=20 - n {
            self.queue[i] = self.queue[i + n].clone();
        }
        self.previous_clips += n;
        if self.previous_clips > 5 {
            self.previous_clips = 5
        }
        self.remaining_clips -= n;
        Ok(())
    }
    fn rewind(&mut self, n: usize) -> Result<(), ()> {
        if n > self.previous_clips {
            return Err(());
        }
        for i in 0..=20 - n {
            self.queue[20 - i] = self.queue[20 - i - n].clone();
        }
        self.previous_clips -= n;
        self.remaining_clips += n;
        Ok(())
    }
    fn current(&self) -> Option<ReceivedClipRequest> {
        self.queue[5].clone()
    }
    fn remove(&mut self, i: usize) -> Result<(), ()> {
        if i < 5 - self.previous_clips || i > 4 + self.remaining_clips {
            return Err(());
        }
        if i < 5 {
            for j in 0..i {
                self.queue[i - j] = self.queue[i - j - 1].clone();
            }
            self.previous_clips -= 1;
        } else {
            for j in i..20 {
                self.queue[j] = self.queue[j + 1].clone();
            }
            self.remaining_clips -= 1;
        }
        Ok(())
    }
    fn enqueue(&mut self, clip: ReceivedClipRequest) -> Result<usize, String> {
        if self.remaining_clips > 10 {
            return Err("queue is full".to_owned());
        }
        self.queue[self.remaining_clips + 5] = Some(clip);
        self.remaining_clips += 1;
        Ok(self.remaining_clips + 4)
    }
    fn clear(&mut self) {
        self.queue = Default::default();
        self.previous_clips = 0;
        self.remaining_clips = 0;
    }
}

#[derive(Deserialize)]
struct CheckClipRequest {
    data: Vec<ClipWithDuration>,
}
#[derive(Deserialize)]
struct ClipWithDuration {
    duration: f32,
    title: String,
}

async fn check_for_clip(
    url: &str,
    auth: &String,
    client_id: &String,
) -> Option<(f32, String, String)> {
    //parse url to remove query parameters
    let url = match Url::parse(url) {
        Ok(a) => a.path().to_owned(),
        Err(_) => {
            println!("no resolvable url");
            return None;
        }
    };
    let clip_id_t: Vec<&str> = url.trim().rsplit_terminator('/').collect();
    let clip_id_t = clip_id_t.first();
    let clip_id = match clip_id_t {
        Some(id) => id.to_owned().to_owned(),
        None => {
            println!("couldn't get clip id");
            return None;
        }
    };
    println!("{}", clip_id);
    let client = reqwest::Client::new();
    let res = match client
        .get(format!("https://api.twitch.tv/helix/clips?id={}", clip_id))
        .header("Authorization".to_owned(), format!("Bearer {}", auth))
        .header("Client-Id".to_owned(), client_id)
        .send()
        .await
    {
        Ok(r) => r.json::<CheckClipRequest>().await,
        Err(_) => {
            println!("twitch api didn't respond");
            return None;
        }
    };
    match res {
        Ok(r) => match r.data.get(0) {
            Some(clip) => Some((clip.duration, clip_id, clip.title.clone())),
            None => {
                println!("no clip returned by twitch");
                None
            }
        },
        Err(_) => {
            println!("couldn't find clip");
            None
        }
    }
}
