use generate_actix_response as responses;
use std::cell::Cell;

#[actix_web::test]
async fn response_macros_preserve_status_message_and_data() {
    let response = responses::generate_created_json_response!(vec!["created"]);
    assert_eq!(response.status().as_u16(), 201);
    let body = actix_web::body::to_bytes(response.into_body())
        .await
        .unwrap();
    let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(
        body,
        serde_json::json!({"status":201,"message":"Created","data":["created"]})
    );

    let response = responses::generate_not_found_json_response_with_message!("Missing");
    assert_eq!(response.status().as_u16(), 404);
    let body = actix_web::body::to_bytes(response.into_body())
        .await
        .unwrap();
    let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(body, serde_json::json!({"status":404,"message":"Missing"}));

    let response = responses::generate_ok_json_response!();
    assert_eq!(response.status().as_u16(), 200);
    let response = responses::generate_ok_json_response_with_message!("Done", 42);
    assert_eq!(response.status().as_u16(), 200);
}

#[test]
fn content_macro_supports_every_form_and_evaluates_arguments_once() {
    let count = Cell::new(0);
    let (code, body) = responses::generate_response_content!({
        count.set(count.get() + 1);
        202
    });
    assert_eq!(count.get(), 1);
    assert_eq!(code.as_u16(), 202);
    assert_eq!(body.message, "Accepted");
    assert_eq!(body.data, None);
    let (_, body) = responses::generate_response_content!(400, "Invalid");
    assert_eq!(body.message, "Invalid");
    let (_, body) = responses::generate_response_content!(200, "Done", 42);
    assert_eq!(body.data, Some(42));
}

#[test]
fn invalid_status_codes_do_not_wrap() {
    for status in [-1, 99, 1000, 65736] {
        assert!(
            std::panic::catch_unwind(|| responses::generate_response_content!(status)).is_err()
        );
    }
}

#[cfg(feature = "utoipa")]
#[test]
fn response_supports_openapi() {
    fn has_schema<T: utoipa::ToSchema>() {}
    has_schema::<responses::StandardResponse<String>>();
}
