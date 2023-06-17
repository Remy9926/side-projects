import { SlashCommandBuilder } from 'discord.js';

module.exports = {
    data: new SlashCommandBuilder()
        .setName("server")
        .setDescription("Information about the current discord server"),
    
    async execute(interaction: any) {
        await interaction.reply(`This server's name is ${interaction.guild.name} and has ${interaction.guild.memberCount} members`);
    }
}