table! {
    customer_orders (id) {
        id -> Int4,
        customer_name -> Varchar,
        item -> Varchar,
        cost -> Float8,
        order_status -> Int4,
    }
}