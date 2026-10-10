use axum::{Json, extract::State, http::StatusCode};
use store::{Store, models::{website::AddWebsite, website_res::AddWebsiteRes}};
use axum::extract::Path;

pub async fn create_website(
    State(store): State<Store>,
    Json(payload): Json<AddWebsite>,
) -> Result<Json<AddWebsiteRes>, (StatusCode, String)> {
//Add middle ware to get loged user id
    let site = AddWebsite {
        url: payload.url,
    };

    let response = store.add_website(&site.url)
    .await
    .map_err(|e|(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(response))
}   

pub async fn fetch_websites(
    State(store ): State<Store>,
    Path(id): Path<String>
) -> Result<Json<Vec<AddWebsiteRes>>, (StatusCode, String)>{
    //Middle ware to verify loged in user
    let response = store.get_website(id).await
    .map_err(|e| (
        StatusCode::INTERNAL_SERVER_ERROR,
        e.to_string()
    ))?;
    Ok(Json(response))
}