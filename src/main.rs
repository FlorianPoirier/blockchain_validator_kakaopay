use std::collections::HashMap;

mod apis;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let url = "https://kapi.kakao.com/v1/payment/ready";
    let body_entries = vec![
        ("cid", "TC0ONETIME"),
        ("partner_order_id", "ORDER_ID_12345"),
        ("partner_user_id", "USER_ID_67890"),
        ("item_name", "Produit de Test"),
        ("quantity", "1"),
        ("total_amount", "1000"),
        ("vat_amount", "100"),
        ("tax_free_amount", "0"),
        ("approval_url", "http://localhost/success"),
        ("cancel_url", "http://localhost/cancel"),
        ("fail_url", "http://localhost/fail"),
    ];

    let body: HashMap<&str, &str> = body_entries.into_iter().collect();


    apis::api::run_post_call(url, body).await.expect("TODO: panic message");

}
