fn main() {
    let response = responses::generate_ok_json_response!(vec!["hello"]);
    assert_eq!(response.status().as_u16(), 200);
    let response = responses::make_json_response!(400, "Invalid");
    assert_eq!(response.status().as_u16(), 400);
    let (_, body) = responses::generate_response_content!(202);
    assert_eq!(body.message, "Accepted");
}
