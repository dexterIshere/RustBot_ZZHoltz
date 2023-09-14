async fn run_pokemon_quizz(ctx: Context, msg: Message, param: String) {
    let m = msg
        .channel_id
        .send_message(&ctx, |m| {
            m.content(&param).components(|c| {
                c.create_action_row(|row| {
                    row.create_select_menu(|menu| {
                        menu.custom_id("animal_select");
                        menu.placeholder("No animal selected");
                        menu.options(|f| {
                            f.create_option(|o| o.label("🐈 meow").value("Cat"));
                            f.create_option(|o| o.label("🐕 woof").value("Dog"));
                            f.create_option(|o| o.label("🐎 neigh").value("Horse"));
                            f.create_option(|o| o.label("🦙 hoooooooonk").value("Alpaca"));
                            f.create_option(|o| o.label("🦀 crab rave").value("Ferris"))
                        })
                    })
                })
            })
        })
        .await;
    m.unwrap();
}

async fn run_flags_quizz(ctx: Context, msg: Message, param: String) {
    let m = msg
        .channel_id
        .send_message(&ctx, |m| {
            m.content(&param).components(|c| {
                c.create_action_row(|row| {
                    row.create_select_menu(|menu| {
                        menu.custom_id("animal_select");
                        menu.placeholder("No animal selected");
                        menu.options(|f| {
                            f.create_option(|o| o.label("🐈 meow").value("Cat"));
                            f.create_option(|o| o.label("🐕 woof").value("Dog"));
                            f.create_option(|o| o.label("🐎 neigh").value("Horse"));
                            f.create_option(|o| o.label("🦙 hoooooooonk").value("Alpaca"));
                            f.create_option(|o| o.label("🦀 crab rave").value("Ferris"))
                        })
                    })
                })
            })
        })
        .await;
    m.unwrap();
}
