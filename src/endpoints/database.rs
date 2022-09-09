use diesel::pg::PgConnection;
use diesel::{prelude::*, Queryable, Insertable};
use rocket::serde::json::{Json, json, Value};
use rocket::serde::{Deserialize, Serialize};
use dotenv::dotenv;
use std::env;

use crate::endpoints::AppDatabase;
use crate::schema::customer_orders;
// Table model
#[derive(Queryable, Serialize)]
pub struct Order {
    pub id: i32,
    pub customer_name: String,
    pub item: String,
    pub cost: f64,
    pub order_status: i32
}

impl Order {
    pub fn to_json_response(self) -> JsonOrder {
        JsonOrder {
            id: self.id,
            customer_name: self.customer_name,
            item: self.item,
            cost: self.cost,
            order_status: self.order_status
        }
    }
}

#[derive(Insertable)]
#[table_name="customer_orders"]
struct NewOrder<'a> {
    pub customer_name: &'a str,
    pub item: &'a str,
    pub cost: &'a f64,
    pub order_status: &'a i32
}

#[derive(Deserialize)]
pub struct NewOrderData {

    pub customer_name: Option<String>,
    pub item: Option<String>,
    pub cost: Option<f64>
}

#[derive(Serialize)]
pub struct JsonOrder {
    pub id: i32,
    pub customer_name: String,
    pub item: String,
    pub cost: f64,
    pub order_status: i32
}

#[post("/database-operation", data="<new_data>")]
pub async fn post_database_operation(db_conn: AppDatabase,
     new_data: Json<NewOrderData>) -> Option<Value> {

        let data = new_data.into_inner();

        //// Import data to database

        let new_customer_data = data.customer_name.unwrap().to_owned();
        let new_item = data.item.unwrap().to_owned();
        let new_cost = data.cost.unwrap();


        db_conn.run(move |conn|import_data(
            new_customer_data.as_str(),
            new_item.as_str(),
            &new_cost, conn))
            .await
            .map(|order| json!(order.to_json_response()));

        db_conn.run(move |conn| export_data(conn))
            .await
            .map(|order| json!(order.to_json_response()))
}


fn import_data(customer_name: &str,
    item: &str, cost: &f64, conn: &PgConnection) -> Option<Order> {

    let order_status = 0;
    let new_order = &NewOrder {
        customer_name,
        item,
        cost,
        order_status: &order_status
    };
    let results = diesel::insert_into(customer_orders::table)
    .values(new_order)
    .get_result::<Order>(conn)
    .expect("Error saving new customer order");
    
    Some(results)
}

fn export_data (conn: &PgConnection) -> Option<Order>{

    let results = customer_orders::table
        .filter(customer_orders::order_status.eq(0))
        .limit(1)
        .load::<Order>(conn)
        .expect("Error loading orders");

    results.into_iter().nth(0)
}
