use serde::Deserialize;
use crate::entity::{username, Credentials, author};
use crate::{request, dto};
use futures::prelude::*;
use seed::fetch;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct RootDto {
    profile: dto::AuthorDto
}

pub fn load<Ms: 'static>(
    credentials: Option<Credentials>,
    username: username::Username<'static>,
    f: fn(Result<author::Author<'static>, (username::Username<'static>, Vec<String>)>) -> Ms,
) -> impl Future<Item=Ms, Error=Ms>  {
    request::new_api_request(
        &format!("profiles/{}", username.as_str()),
        credentials.as_ref()
    )
        .fetch_json_data(move |data_result: fetch::ResponseDataResult<RootDto>| {
            f(data_result
                .map(move |root_dto| root_dto.profile.into_author(credentials))
                .map_err(request::fail_reason_into_errors)
                .map_err(move |errors| (username, errors))
            )
        })
}