import 'dotenv/config';
import { Client, Collection, Events, GatewayIntentBits } from 'discord.js';
import * as sayHello from './commands/hello';
import * as fs from 'fs';
import * as path from 'path';

async function setCommandInCollection(commandFilePath: string) {
    await import(commandFilePath)
    .then( (response) => {
        var command = response;
        if ('data' in command && 'execute' in command) {
            client.commands.set(command.data.name, command);
        }
        else {
            console.log(`File could not be registered as a command`);
        }
    })
}

const client = new Client({
    intents: [
        GatewayIntentBits.Guilds,
        GatewayIntentBits.GuildMessages,
        GatewayIntentBits.GuildMembers,
        GatewayIntentBits.DirectMessages,
    ]
})

client.commands = new Collection();

const commandsPath = path.join(__dirname + "/commands");
const commandFiles = fs.readdirSync(commandsPath);

for (let file in commandFiles) {
    const filePath = path.join(commandsPath + "/" + commandFiles[file]);
    setCommandInCollection(filePath);
}


client.login(process.env.DISCORD_TOKEN)
    .then(() => {
    console.log("logged in!");
})

client.on("messageCreate", (message) => {
    if (!message.author.bot && message.channelId === '1118242285205860402') {
        console.log(message);
        message.channel.send("i got your message!");
    }
})