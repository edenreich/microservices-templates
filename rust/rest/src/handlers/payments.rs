use crate::{database::mocked_database::Database, models::payment::Payment};
use serde_json::{json, Value};
use std::sync::{Arc, Mutex};
use warp::{reply::Json, Filter};

use crate::with_mocked_db;

pub fn payments_filter(
    db: Arc<Mutex<Database>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let base = warp::path("api")
        .and(warp::path("v1"))
        .and(warp::path("payments"));

    let list = warp::get()
        .and(warp::path::end())
        .and(with_mocked_db(db.clone()))
        .and_then(get_payments);

    let find = warp::get()
        .and(with_mocked_db(db.clone()))
        .and(warp::path::param())
        .and(warp::path::end())
        .and_then(get_payment)
        .recover(|err: warp::Rejection| async move {
            if err.is_not_found() {
                Ok(warp::reply::with_status(
                    warp::reply::json(&json!({
                        "message": "Payment not found"
                    })),
                    warp::http::StatusCode::NOT_FOUND,
                ))
            } else {
                Err(err)
            }
        });

    let create = warp::post()
        .and(with_mocked_db(db.clone()))
        .and(warp::body::json())
        .and(warp::path::end())
        .and_then(create_payment);

    let update = warp::put()
        .and(with_mocked_db(db.clone()))
        .and(warp::body::json())
        .and_then(update_payment)
        .recover(|err: warp::Rejection| async move {
            if err.is_not_found() {
                Ok(warp::reply::with_status(
                    warp::reply::json(&json!({
                        "message": "Payment not found"
                    })),
                    warp::http::StatusCode::NOT_FOUND,
                ))
            } else {
                Err(err)
            }
        });

    base.and(list.or(find).or(create).or(update))
}

async fn get_payments(db: Arc<Mutex<Database>>) -> Result<Json, warp::Rejection> {
    let db = db.lock().unwrap();
    Ok(warp::reply::json(&db.get_payments()))
}

async fn get_payment(
    db: Arc<Mutex<Database>>,
    id: u32,
) -> Result<impl warp::Reply, warp::Rejection> {
    let db = db.lock().unwrap();
    let payments = db.get_payments();
    let payment = payments.iter().find(|p| p.id == id);

    match payment {
        Some(p) => Ok(warp::reply::json(&p)),
        None => Err(warp::reject::not_found()),
    }
}

async fn create_payment(
    db: Arc<Mutex<Database>>,
    payment: Payment,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut db = db.lock().unwrap();
    db.create_payment(payment);
    Ok("Payment created!".to_string())
}

async fn update_payment(
    db: Arc<Mutex<Database>>,
    data: Value,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut db = db.lock().unwrap();
    let id = data["id"].as_u64().unwrap() as u32;
    let payment: Payment = match db.get_payment(id) {
        Some(payment) => payment,
        None => return Err(warp::reject::not_found()),
    };
    let payment = Payment {
        id: payment.id,
        amount: data["amount"].as_u64().unwrap() as u32,
        currency: data["currency"].as_str().unwrap().to_string(),
        status: data["status"].as_str().unwrap().to_string(),
    };
    db.update_payment(payment);
    Ok("Payment updated!".to_string())
}
