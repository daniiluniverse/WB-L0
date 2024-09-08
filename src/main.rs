mod models;
mod db;
pub mod test_api;

use axum::{routing::{get, post}, Router,};
use std::sync::Arc;
use tokio_postgres::{NoTls, Error, Transaction};
use crate::db::{get_status, add_order};
use log::{debug, error, info};
use tokio::sync::Mutex;

//type SharedState = Arc<tokio_postgres::Client>;
type SharedState = Arc<Mutex<tokio_postgres::Client>>;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {


    env_logger::init();

    info!("Запуск сервера...");


    // Подключение к базе данных
    let (mut client, connection) =
        tokio_postgres::connect("host=127.0.0.1 user=postgres dbname=orders_db password=110199", NoTls).await?;

    //Запускаем асинхронное соединение
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            error!("Ошибка подключения к базе данных: {}", e);
        }
    });

    debug!("База данных подключена!");

    // Инициализация глобального состояния с клиентом базы данных
let shared_state: SharedState = Arc::new(Mutex::new(client));

    // Создаем маршруты
    let app = Router::new()
        .route("/", get(get_status))
        .route("/add_order", post(add_order))
        .with_state(shared_state);

    let address = "127.0.0.1:3000";

     info!("Сервер запущен на {}", address);



    // Запуск сервера
     // Запуск сервера
    let listener = tokio::net::TcpListener::bind(address).await;
    match listener {
        Ok(listener) => {
            info!("Сервер успешно подключен к {}", address);
            if let Err(e) = axum::serve(listener, app).await {
                error!("Ошибка при запуске сервера: {}", e);
            }
        }
        Err(e) => {
            error!("Не удалось подключиться к адресу {}: {}", address, e);
        }
    }



    Ok(())
}

















































// mod models;
// use axum::{
//     extract::State,
//     routing::{get, post},
//     Json, Router,
// };
// use serde_json::json;
// use std::sync::{Arc, Mutex};
// use std::net::SocketAddr;
// use crate::models::Order;
// use tokio_postgres::{NoTls, Error};
//
// type SharedState = Arc<Mutex<Vec<Order>>>;
//
// #[tokio::main]
// async fn main() -> Result<(), Error> {
//
//
//
//
//
//     // Инициализация глобального состояния с пустым списком заказов
//     let shared_state: SharedState = Arc::new(Mutex::new(Vec::new()));
//
//     // Создаем маршруты
//     let app = Router::new()
//         .route("/status", get(get_status))
//         .route("/add_order", post(add_order))
//         .with_state(shared_state);
//
//     // Запуск сервера
//
//     let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
//     axum::serve(listener, app).await.unwrap();
//
//
//
// }
//
// // Обработчик для получения списка заказов
// async fn get_status(State(state): State<SharedState>) -> Json<Vec<Order>> {
//     let orders = state.lock().unwrap();
//     Json(orders.clone())
// }
//
// // Обработчик для добавления нового заказа
// async fn add_order(
//     State(state): State<SharedState>,
//     Json(new_order): Json<Order>,
// ) -> Json<serde_json::Value> {
//     let mut orders = state.lock().unwrap();
//     orders.push(new_order);
//     Json(json!({ "status": "Order added successfully" }))
// }