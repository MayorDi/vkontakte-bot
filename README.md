VKontakte-bot
===

`VKontakte-bot` - это маленькая библиотека, создана в целях практики.

## Example
``` rust
let access_token = "your token";
let group_id = 0; // id of your group

let api_settings = ApiSettings::new()
    .set_access_token(access_token)
    .set_version("5.99");

let mut vk_bot = VkBot::new(group_id, api_settings);

vk_bot.command("/lang", |ctx| {
    ctx.reply("Rust").unwrap();
});

vk_bot.command("/name", |ctx| {
    ctx.reply("Bob").unwrap();
});

vk_bot.init().unwrap().run().unwrap();
```

### Use regex
``` rust
let access_token = "your token";
let group_id = 0; // id of your group

let api_settings = ApiSettings::new()
    .set_access_token(access_token)
    .set_version("5.99");

let mut vk_bot = VkBot::new(group_id, api_settings);

vk_bot.command(r"/num (?P<num>\d*)", |ctx| {
    let res = &ctx.captures["num"];
    ctx.reply(format!("num: {}", res).as_str()).unwrap();
});

vk_bot.init().unwrap().run().unwrap();
```
