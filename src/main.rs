mod action;
mod app;
mod components;
mod content;
mod db;
mod event;
mod models;
mod screens;
mod tui;
mod update;
mod validator;
mod view;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut app = app::App::new()?;
    app.run().await
}
