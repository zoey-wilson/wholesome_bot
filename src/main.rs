extern crate serenity;
extern crate rand;
extern crate reqwest;

use rand::Rng;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

use serenity::{
    model::{channel::Message, gateway::Ready, user::OnlineStatus, gateway::Game},
    prelude::*,
};

//handler recieves messages and does something if they match a command
struct Handler;
impl EventHandler for Handler {
    fn message(&self, _: Context, msg: Message) {

        if msg.content.starts_with("!help") { // If the user types help it brings up a list of commands.
            println!("Sending normal command list");
            if let Err(why) = msg.channel_id.say("The wholesome bot has the following commands:\n
            !breathe or !b - Sends a link to a gif that should help you calm down\n
            !wholesome or !wm - Sends a wholesome message back to you!") {
                println!("Error sending message: {:?}", why); //sends error message to console if it fails
            }
        }

        // Sends link of anxiety breathing gif in chat
        if msg.content.starts_with("!breathe") || msg.content.starts_with("!b") { // if the user types the command it sends the message.
            println!("Sending link to breathing gif");
            if let Err(why) = msg.channel_id.say("https://www.zoeywilson.co.uk/LooseFilesPlsNoRead/RandomThings/DiscordBotResources/breathing_gif_from_robert_duff_www.duffthepsych.com.gif") {
                println!("Error sending message: {:?}", why); //sends error message to console if it fails
            }
        }


        if msg.content.starts_with("!wholesome") || msg.content.starts_with("!wm"){

            // Opens wholesome.txt and puts it into an array
            println!("Adding wholesome.txt to array");
            let _wholesome_file = File::open("wholesome.txt").unwrap();
            let mut _wholesome_choices = Vec::new();
            for _wholesome_line in BufReader::new(_wholesome_file).lines() {
                _wholesome_choices.push(_wholesome_line.unwrap());
            }

            // Selects a random index of the wholesome message array and sends the message with the randomised string from the array
            println!("Choosing wholesome string and sending!");
            let mut rng = rand::thread_rng();
            let _wholesome_i = rng.gen_range(0, _wholesome_choices.len());
            if let Err(why) = msg.channel_id.say( _wholesome_choices.get(_wholesome_i).unwrap()) {
                println!("Error sending message: {:?}", why);
            }
        }
    }


    fn ready(&self, context: Context, ready: Ready) {  //Notifies the console that the bot is launched as well as setting the game and online status
        let game = Game::playing("Type !help for commands - Take care of yourselves <3");
        let status = OnlineStatus::Online;

        context.set_presence(Some(game), status);
        println!("{} is connected!", ready.user.name);
    }
}

fn main() {

    // Downloads the latest version of wholesome.txt
    println!("Downloading wholesome.txt");
    let mut dl_wholesome_resp = reqwest::get("https://www.zoeywilson.co.uk/LooseFilesPlsNoRead/RandomThings/DiscordBotResources/wholesome.txt").expect("request failed");
    let mut dl_wholesome_out = File::create("wholesome.txt").expect("failed to create wholesome.txt");
    io::copy(&mut dl_wholesome_resp, &mut dl_wholesome_out).expect("failed to copy content to wholesome.txt");

    // creates the bot client
    let token = "---------------------DISCORD-TOKEN-HERE---------------------";
    let mut client = Client::new(&token, Handler).expect("Err creating client");

    // launches the bot client shard
    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}