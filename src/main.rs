use teloxide::{prelude::*, /*dispatching::dialogue::InMemStorage, */utils::command::BotCommands, Bot};
// use teloxide::{types::Message, Bot};
// use teloxide::{
//     // dispatching::{dialogue, UpdateHandler},
//     // types::{InlineKeyboardButton, InlineKeyboardMarkup},
// };
// use dptree::{prelude::*, case};

use dotenv::dotenv;

mod write_pr;
use crate::write_pr::*;

// type MyDialogue = Dialogue<State, InMemStorage<State>>;
type _HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;


#[tokio::main]
async fn main() {
    dotenv().ok();

    let bot = Bot::from_env();

    println!("Starting bot...");

    // let command_handler = teloxide::filter_command::<Command, _>()
    // .branch(case![Command::Start].endpoint(start))
    // .branch(case![Command::Add].endpoint(add))
    // .branch(case![Command::Help].endpoint(help));

    // Dispatcher::builder(bot, command_handler)
    //     .enable_ctrlc_handler()
    //     .build()    
    //     .dispatch()
    //     .await;

    Command::repl(bot, answer).await;
}


// #[derive(Clone, Default)]
// pub enum State {
//     #[default]
//     Start,
//     ToDoBranch
// }


#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase",description = "Бот поддерживает данные команды:")]
enum Command {
    #[command(description = "текст при запуске")]
    Start,
    #[command(description = "выводит список команд")]
    Help,
    #[command(description = "добавить новую задачу")]
    Add(String),
    #[command(description = "показать список задач")]
    View,
    #[command(description = "удаляет задачу из списка")]
    Done(usize),
    #[command(description = "очищает список задач")]
    Clean
}


async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
        Command::Start => {
            bot.send_message(msg.chat.id,
                "Привет! Этот бот может делать todo-list, для подробностей воспользуйтесь /help").await?
        }
        Command::Add(task) => {
            let _ = add_task(msg.chat.id.to_string(), task.clone());
            bot.send_message(msg.chat.id, format!("Добавлено \"{task}\"")).await?
        }
        Command::View => {
            bot./*parse_mode(teloxide::types::ParseMode::MarkdownV2).*/send_message(msg.chat.id,
                read_tasks(msg.chat.id.to_string())).await?
        }
        Command::Done(number) => {
            // let t = mark(msg.chat.id.to_string(), number);
            // match t {
            //     Ok(_) => {bot.send_message(msg.chat.id, format!("Удалена задача {number}")).await?}
            //     _ => {bot.send_message(msg.chat.id, "Произошла ошибка при удалении").await?}
            // }
            let _ = mark(msg.chat.id.to_string(), number);
            bot.send_message(msg.chat.id, format!("Удалена задача №{number}")).await?
        }
        Command::Clean => {
            let _ = clear(msg.chat.id.to_string());
            bot.send_message(msg.chat.id, "Список очищен").await?
        }
    };

    Ok(())
}

// fn schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
//     use dptree::case;

//     let command_handler = teloxide::filter_command::<Command, _>()
//         .branch(case![Command::Start].endpoint(start))
//         .branch(case![Command::Add].endpoint(add));

//     // let message_handler = Update::filter_message()
//     //     .branch(command_handler)

//     dialogue::enter::<Update, InMemStorage<State>, State, _>()
//         .branch(command_handler)
// }

async fn _start(bot: Bot, msg: Message) -> _HandlerResult {
    bot.send_message(msg.chat.id, "Hi").await?;
    // dialogue.update(State::ReceiveFullName).await?;
    Ok(())
}

async fn _add(bot: Bot, msg: Message) -> _HandlerResult {
    bot.send_message(msg.chat.id, "Adding...").await?;
    // dialogue.update(State::ReceiveFullName).await?;
    Ok(())
}

async fn _help(bot: Bot, msg: Message) -> _HandlerResult {
    bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
    // dialogue.update(State::ReceiveFullName).await?;
    Ok(())
}