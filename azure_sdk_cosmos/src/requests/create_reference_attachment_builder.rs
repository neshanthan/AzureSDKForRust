use crate::clients::{CosmosUriBuilder, ResourceType};
use crate::prelude::*;
use crate::AttachmentClient;
use crate::AttachmentClientRequired;
use azure_sdk_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_sdk_core::prelude::*;
use azure_sdk_core::{No, ToAssign, Yes};
use hyper::StatusCode;
use std::convert::TryInto;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct CreateReferenceAttachmentBuilder<'a, 'b, CUB, ContentTypeSet, MediaSet>
where
    ContentTypeSet: ToAssign,
    MediaSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    attachment_client: &'a AttachmentClient<'a, CUB>,
    p_content_type: PhantomData<ContentTypeSet>,
    p_media: PhantomData<MediaSet>,
    content_type: Option<&'b str>,
    media: Option<&'b str>,
    user_agent: Option<&'b str>,
    activity_id: Option<&'b str>,
    consistency_level: Option<ConsistencyLevel<'b>>,
}

impl<'a, 'b, CUB> CreateReferenceAttachmentBuilder<'a, 'b, CUB, No, No>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    pub(crate) fn new(
        attachment_client: &'a AttachmentClient<'a, CUB>,
    ) -> CreateReferenceAttachmentBuilder<'a, 'b, CUB, No, No> {
        CreateReferenceAttachmentBuilder {
            attachment_client,
            p_content_type: PhantomData {},
            content_type: None,
            p_media: PhantomData {},
            media: None,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
        }
    }
}

impl<'a, 'b, CUB, ContentTypeSet, MediaSet> AttachmentClientRequired<'a, CUB>
    for CreateReferenceAttachmentBuilder<'a, 'b, CUB, ContentTypeSet, MediaSet>
where
    ContentTypeSet: ToAssign,
    MediaSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn attachment_client(&self) -> &'a AttachmentClient<'a, CUB> {
        self.attachment_client
    }
}

//get mandatory no traits methods

//set mandatory no traits methods
impl<'a, 'b, CUB, MediaSet> ContentTypeRequired<'b>
    for CreateReferenceAttachmentBuilder<'a, 'b, CUB, Yes, MediaSet>
where
    MediaSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn content_type(&self) -> &'b str {
        self.content_type.unwrap()
    }
}

impl<'a, 'b, CUB, ContentTypeSet> MediaRequired<'b>
    for CreateReferenceAttachmentBuilder<'a, 'b, CUB, ContentTypeSet, Yes>
where
    ContentTypeSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn media(&self) -> &'b str {
        self.media.unwrap()
    }
}

impl<'a, 'b, CUB, ContentTypeSet, MediaSet> UserAgentOption<'b>
    for CreateReferenceAttachmentBuilder<'a, 'b, CUB, ContentTypeSet, MediaSet>
where
    ContentTypeSet: ToAssign,
    MediaSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn user_agent(&self) -> Option<&'b str> {
        self.user_agent
    }
}

impl<'a, 'b, CUB, ContentTypeSet, MediaSet> ActivityIdOption<'b>
    for CreateReferenceAttachmentBuilder<'a, 'b, CUB, ContentTypeSet, MediaSet>
where
    ContentTypeSet: ToAssign,
    MediaSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn activity_id(&self) -> Option<&'b str> {
        self.activity_id
    }
}

impl<'a, 'b, CUB, ContentTypeSet, MediaSet> ConsistencyLevelOption<'b>
    for CreateReferenceAttachmentBuilder<'a, 'b, CUB, ContentTypeSet, MediaSet>
where
    ContentTypeSet: ToAssign,
    MediaSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn consistency_level(&self) -> Option<ConsistencyLevel<'b>> {
        self.consistency_level.clone()
    }
}

impl<'a, 'b, CUB, MediaSet> ContentTypeSupport<'b>
    for CreateReferenceAttachmentBuilder<'a, 'b, CUB, No, MediaSet>
where
    MediaSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = CreateReferenceAttachmentBuilder<'a, 'b, CUB, Yes, MediaSet>;

    #[inline]
    fn with_content_type(self, content_type: &'b str) -> Self::O {
        CreateReferenceAttachmentBuilder {
            attachment_client: self.attachment_client,
            p_content_type: PhantomData {},
            p_media: PhantomData {},
            content_type: Some(content_type),
            media: self.media,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, CUB, ContentTypeSet> MediaSupport<'b>
    for CreateReferenceAttachmentBuilder<'a, 'b, CUB, ContentTypeSet, No>
where
    ContentTypeSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = CreateReferenceAttachmentBuilder<'a, 'b, CUB, ContentTypeSet, Yes>;

    #[inline]
    fn with_media(self, media: &'b str) -> Self::O {
        CreateReferenceAttachmentBuilder {
            attachment_client: self.attachment_client,
            p_content_type: PhantomData {},
            p_media: PhantomData {},
            content_type: self.content_type,
            media: Some(media),
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, CUB, ContentTypeSet, MediaSet> UserAgentSupport<'b>
    for CreateReferenceAttachmentBuilder<'a, 'b, CUB, ContentTypeSet, MediaSet>
where
    ContentTypeSet: ToAssign,
    MediaSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = CreateReferenceAttachmentBuilder<'a, 'b, CUB, ContentTypeSet, MediaSet>;

    #[inline]
    fn with_user_agent(self, user_agent: &'b str) -> Self::O {
        CreateReferenceAttachmentBuilder {
            attachment_client: self.attachment_client,
            p_content_type: PhantomData {},
            p_media: PhantomData {},
            content_type: self.content_type,
            media: self.media,
            user_agent: Some(user_agent),
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, CUB, ContentTypeSet, MediaSet> ActivityIdSupport<'b>
    for CreateReferenceAttachmentBuilder<'a, 'b, CUB, ContentTypeSet, MediaSet>
where
    ContentTypeSet: ToAssign,
    MediaSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = CreateReferenceAttachmentBuilder<'a, 'b, CUB, ContentTypeSet, MediaSet>;

    #[inline]
    fn with_activity_id(self, activity_id: &'b str) -> Self::O {
        CreateReferenceAttachmentBuilder {
            attachment_client: self.attachment_client,
            p_content_type: PhantomData {},
            p_media: PhantomData {},
            content_type: self.content_type,
            media: self.media,
            user_agent: self.user_agent,
            activity_id: Some(activity_id),
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, CUB, ContentTypeSet, MediaSet> ConsistencyLevelSupport<'b>
    for CreateReferenceAttachmentBuilder<'a, 'b, CUB, ContentTypeSet, MediaSet>
where
    ContentTypeSet: ToAssign,
    MediaSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = CreateReferenceAttachmentBuilder<'a, 'b, CUB, ContentTypeSet, MediaSet>;

    #[inline]
    fn with_consistency_level(self, consistency_level: ConsistencyLevel<'b>) -> Self::O {
        CreateReferenceAttachmentBuilder {
            attachment_client: self.attachment_client,
            p_content_type: PhantomData {},
            p_media: PhantomData {},
            content_type: self.content_type,
            media: self.media,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: Some(consistency_level),
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, 'b, CUB> CreateReferenceAttachmentBuilder<'a, 'b, CUB, Yes, Yes>
where
    CUB: CosmosUriBuilder,
{
    pub async fn execute(
        &self,
    ) -> Result<crate::responses::CreateReferenceAttachmentResponse, AzureError> {
        let mut req = self.attachment_client.main_client().prepare_request(
            &format!(
                "dbs/{}/colls/{}/docs/{}/attachments",
                self.attachment_client.database_name().name(),
                self.attachment_client.collection_name().name(),
                self.attachment_client.document_name().name(),
            ),
            hyper::Method::POST,
            ResourceType::Attachments,
        );

        // add trait headers
        req = UserAgentOption::add_header(self, req);
        req = ActivityIdOption::add_header(self, req);
        req = ConsistencyLevelOption::add_header(self, req);

        req = crate::add_partition_keys_header(
            self.attachment_client.document_client().partition_keys(),
            req,
        );

        // create serialized request
        #[derive(Debug, Clone, Serialize)]
        struct _Request<'r> {
            pub id: &'r str,
            #[serde(rename = "contentType")]
            pub content_type: &'r str,
            pub media: &'r str,
        }

        let request = serde_json::to_string(&_Request {
            id: self.attachment_client.attachment_name().name(),
            content_type: ContentTypeRequired::content_type(self),
            media: self.media(),
        })?;

        req = req.header(http::header::CONTENT_TYPE, "application/json");
        req = req.header(http::header::CONTENT_LENGTH, request.len());
        let req = req.body(hyper::Body::from(request))?;
        debug!("req == {:#?}", req);

        let (headers, whole_body) = check_status_extract_headers_and_body(
            self.attachment_client.hyper_client().request(req),
            StatusCode::CREATED,
        )
        .await?;

        debug!("\nheaders == {:?}", headers);
        debug!("\nwhole body == {:#?}", whole_body);

        Ok((&headers, &whole_body as &[u8]).try_into()?)
    }
}
