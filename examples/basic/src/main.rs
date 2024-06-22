use vkontakte_bot::{api::ApiSettings, vkbot::VkBot};

fn main() {
    let access_token = "your token";
    let group_id = 0; // id of your group

    let api_settings = ApiSettings::new(access_token, "5.99");

    let mut vk_bot = VkBot::new(group_id, api_settings);

    vk_bot.command("/hello", |ctx| {
        ctx.reply("hi!!!!!!").unwrap();
    });

    vk_bot.init().unwrap().run().unwrap();
}
