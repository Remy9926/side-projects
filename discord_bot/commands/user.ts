import { SlashCommandBuilder } from 'discord.js';

module.exports = {
    data: new SlashCommandBuilder()
        .setName("user")
        .setDescription("Provides information about the user"),

    async execute(interaction: any) {
        await interaction.reply(`The user who called the command is ${interaction.user.username} who joined this server at ${interaction.user.joinedAt}`);
    }
}