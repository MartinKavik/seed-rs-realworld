use serde::Deserialize;
use crate::{form::article_editor as form, session, article, request, dto};
use futures::prelude::*;
use seed::fetch;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RootDto {
    article: dto::article::ArticleDTO
}

pub fn update_article<Ms: 'static>(
    session: &session::Session,
    valid_form: &form::ValidForm,
    slug: &article::slug::Slug,
    f: fn(Result<article::Article, Vec<form::Problem>>) -> Ms
) -> impl Future<Item=Ms, Error=Ms>  {
    let session = session.clone();

    request::new_api_request(
        &format!("articles/{}", slug.as_str()),
        session.viewer().map(|viewer| &viewer.credentials)
    )
        .method(fetch::Method::Put)
        .send_json(&valid_form.dto())
        .fetch_json_data(move |data_result: fetch::ResponseDataResult<RootDto>| {
            f(data_result
                .map_err(fail_reason_to_problems)
                .and_then(move |root_dto| {
                    root_dto.article.try_into_article(session)
                        .map_err(|error| vec![form::Problem::new_server_error(error)])
                })
            )
        })
}

pub fn fail_reason_to_problems(fail_reason: fetch::FailReason<RootDto>) -> Vec<form::Problem> {
    string_errors_to_problems(request::fail_reason_into_errors(fail_reason))
}

pub fn string_errors_to_problems(errors: Vec<String>) -> Vec<form::Problem> {
    errors.into_iter().map(form::Problem::new_server_error).collect()
}