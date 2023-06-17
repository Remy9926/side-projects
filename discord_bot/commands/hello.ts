import { ButtonBuilder, ButtonStyle, SlashCommandBuilder } from 'discord.js';

module.exports = {
    data: new SlashCommandBuilder()
        .setName("hello")
        .setDescription("Bot says hello"),
    
    async execute(interaction: any) {
        await interaction.reply("Hello!");
    }
}