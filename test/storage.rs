use recorder::app;
use axum::http::{Request, StatusCode};
use tower::ServiceExt;

#[tokio::test]
async fn test_get_storage_settings_ok() {
    let (router, _) = app::build_app().await;

    let response = router
        .oneshot(Request::builder().uri("/api/storage/settings").body(axum::body::Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
