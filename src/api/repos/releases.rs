use super::*;
use crate::from_response::FromResponse;
use crate::{body::OctoBody, models::repos::Asset};

/// Handler for GitHub's releases API.
///
/// Created with [`RepoHandler::releases`].
pub struct ReleasesHandler<'octo, 'r> {
    parent: &'r RepoHandler<'octo>,
}

impl<'octo, 'r> ReleasesHandler<'octo, 'r> {
    pub(crate) fn new(parent: &'r RepoHandler<'octo>) -> Self {
        Self { parent }
    }

    /// Creates a new [`ListReleasesBuilder`] that can be configured to filter
    /// listing releases.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let page = octocrab.repos("owner", "repo")
    ///     .releases()
    ///     .list()
    ///     // Optional Parameters
    ///     .per_page(100)
    ///     .page(5u32)
    ///     // Send the request
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list(&self) -> ListReleasesBuilder<'_, '_, '_> {
        ListReleasesBuilder::new(self)
    }

    /// Creates a new [`CreateReleaseBuilder`] with `tag_name`.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let page = octocrab.repos("owner", "repo")
    ///     .releases()
    ///     .create("v1.0.0")
    ///     // Optional Parameters
    ///     .target_commitish("main")
    ///     .name("Version 1.0.0")
    ///     .body("Announcing 1.0.0!")
    ///     .draft(false)
    ///     .prerelease(false)
    ///     // Send the request
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn create<'t>(
        &self,
        tag_name: &'t (impl AsRef<str> + ?Sized),
    ) -> CreateReleaseBuilder<'_, '_, '_, 't, '_, '_, '_> {
        CreateReleaseBuilder::new(self, tag_name.as_ref())
    }

    /// Creates a new [`UpdateReleaseBuilder`] with `release_id`.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let release = octocrab.repos("owner", "repo")
    ///     .releases()
    ///     .update(1)
    ///     // Optional Parameters
    ///     .tag_name("v1.0.0")
    ///     .target_commitish("main")
    ///     .name("Version 1.0.0")
    ///     .body("Announcing 1.0.0!")
    ///     .draft(false)
    ///     .prerelease(false)
    ///     // Send the request
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn update(&self, release_id: u64) -> UpdateReleaseBuilder<'_, '_, '_, '_, '_, '_, '_> {
        UpdateReleaseBuilder::new(self, release_id)
    }

    /// Fetches a single asset by its ID.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let asset = octocrab::instance()
    ///     .repos("owner", "repo")
    ///     .releases()
    ///     .get_asset(42u64.into())
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    #[deprecated(note = "use repos::ReleaseAssetsHandler::get instead")]
    pub async fn get_asset(&self, asset_id: u64) -> crate::Result<models::repos::Asset> {
        self.parent.release_assets().get(asset_id).await
    }

    /// Gets the latest release.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let release = octocrab::instance()
    ///     .repos("owner", "repo")
    ///     .releases()
    ///     .get_latest()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_latest(&self) -> crate::Result<models::repos::Release> {
        let route = format!(
            "/repos/{owner}/{repo}/releases/latest",
            owner = self.parent.owner,
            repo = self.parent.repo,
        );

        self.parent.crab.get(route, None::<&()>).await
    }

    /// Gets the release using its tag.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let release = octocrab::instance()
    ///     .repos("owner", "repo")
    ///     .releases()
    ///     .get_by_tag("v1.0.0")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_by_tag(&self, tag: &str) -> crate::Result<models::repos::Release> {
        let route = format!(
            "/repos/{owner}/{repo}/releases/tags/{tag}",
            owner = self.parent.owner,
            repo = self.parent.repo,
            tag = tag,
        );

        self.parent.crab.get(route, None::<&()>).await
    }

    /// Gets the release using its id.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let release = octocrab::instance()
    ///     .repos("owner", "repo")
    ///     .releases()
    ///     .get(3)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self, number: u64) -> Result<models::repos::Release> {
        let route = format!(
            "/repos/{owner}/{repo}/releases/{number}",
            owner = self.parent.owner,
            repo = self.parent.repo,
            number = number,
        );

        self.parent.crab.get(route, None::<&()>).await
    }

    /// Generates [`crate::models::repos::ReleaseNotes`] which describe
    /// a [`crate::models::repos::Release`]
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let release_notes = octocrab::instance()
    ///     .repos("owner", "repo")
    ///     .releases()
    ///     .generate_release_notes("0.1.0")
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn generate_release_notes<'tag_name>(
        &self,
        tag_name: &'tag_name (impl AsRef<str> + ?Sized),
    ) -> GenerateReleaseNotesBuilder<'_, '_, '_, 'tag_name, '_, '_, '_> {
        GenerateReleaseNotesBuilder::new(self, tag_name.as_ref())
    }

    /// Upload an [`crate::models::repos::Asset`] associated with
    /// a [`crate::models::repos::Release`]
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let file_path = std::path::Path::new("/tmp/my_asset.tar.gz");
    /// let file_size = unwrap!(std::fs::metadata(file_path)).len();
    /// let file = unwrap!(tokio::fs::File::open(file).await);
    /// let stream = tokio_util::codec::FramedRead::new(file, tokio_util::codec::BytesCodec::new());
    /// let asset = octocrab::instance()
    ///     .repos("owner", "repo")
    ///     .releases()
    ///     .upload_asset(1, "my_asset.tar.gz", stream)
    ///     .label("My Awesome Asset")
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn upload_asset<'asset_name>(
        &self,
        id: u64,
        asset_name: &'asset_name (impl AsRef<str> + ?Sized),
        body: Bytes,
    ) -> UploadAssetBuilder<'_, '_, '_, 'asset_name, '_> {
        UploadAssetBuilder::new(self, id, asset_name.as_ref(), body)
    }

    /// Creates a new [`ListReleaseAssetsBuilder`] that can be configured to filter
    /// listing release assetss.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let page = octocrab.repos("owner", "repo")
    ///     .releases()
    ///     .assets()
    ///     // Optional Parameters
    ///     .per_page(100)
    ///     .page(5u32)
    ///     // Send the request
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn assets(&self) -> ListReleaseAssetsBuilder<'_, '_, '_> {
        ListReleaseAssetsBuilder::new(self)
    }

    /// Streams the binary contents of an asset.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// use futures_util::StreamExt;
    ///
    /// let mut stream = octocrab::instance()
    ///     .repos("owner", "repo")
    ///     .releases()
    ///     .stream_asset(42u64.into())
    ///     .await?;
    ///
    /// while let Some(chunk) = stream.next().await {
    ///     println!("{:?}", chunk);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "stream")]
    #[cfg_attr(docsrs, doc(cfg(feature = "stream")))]
    #[deprecated(note = "use repos::ReleaseAssetsHandler::stream instead")]
    pub async fn stream_asset(
        &self,
        asset_id: AssetId,
    ) -> crate::Result<impl futures_core::Stream<Item = crate::Result<bytes::Bytes>>> {
        self.parent.release_assets().stream(asset_id).await
    }
}

/// A builder pattern struct for listing releases.
///
/// created by [`ReleasesHandler::list`]
#[derive(serde::Serialize)]
pub struct ListReleasesBuilder<'octo, 'r1, 'r2> {
    #[serde(skip)]
    handler: &'r2 ReleasesHandler<'octo, 'r1>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r1, 'r2> ListReleasesBuilder<'octo, 'r1, 'r2> {
    pub(crate) fn new(handler: &'r2 ReleasesHandler<'octo, 'r1>) -> Self {
        Self {
            handler,
            per_page: None,
            page: None,
        }
    }

    /// Results per page (max 100).
    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.per_page = Some(per_page.into());
        self
    }

    /// Page number of the results to fetch.
    pub fn page(mut self, page: impl Into<u32>) -> Self {
        self.page = Some(page.into());
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> crate::Result<crate::Page<crate::models::repos::Release>> {
        let route = format!(
            "/repos/{owner}/{repo}/releases",
            owner = self.handler.parent.owner,
            repo = self.handler.parent.repo
        );
        self.handler.parent.crab.get(route, Some(&self)).await
    }
}

/// A builder pattern struct for creating releases.
///
/// created by [`ReleasesHandler::create`].
#[derive(serde::Serialize)]
pub struct CreateReleaseBuilder<'octo, 'repos, 'handler, 'tag_name, 'target_commitish, 'name, 'body>
{
    #[serde(skip)]
    handler: &'handler ReleasesHandler<'octo, 'repos>,
    tag_name: &'tag_name str,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_commitish: Option<&'target_commitish str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'name str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<&'body str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    draft: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prerelease: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    make_latest: Option<MakeLatest>,
}

#[derive(Debug, Clone, Copy, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MakeLatest {
    True,
    False,
    Legacy,
}

impl std::fmt::Display for MakeLatest {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let text = match self {
            Self::False => "false",
            Self::True => "true",
            Self::Legacy => "legacy",
        };

        f.write_str(text)
    }
}

impl<'octo, 'repos, 'handler, 'tag_name, 'target_commitish, 'name, 'body>
    CreateReleaseBuilder<'octo, 'repos, 'handler, 'tag_name, 'target_commitish, 'name, 'body>
{
    pub(crate) fn new(
        handler: &'handler ReleasesHandler<'octo, 'repos>,
        tag_name: &'tag_name str,
    ) -> Self {
        Self {
            handler,
            tag_name,
            target_commitish: None,
            name: None,
            body: None,
            draft: None,
            prerelease: None,
            make_latest: None,
        }
    }

    /// Specifies the commitish value that determines where the Git tag is
    /// created from. Can be any branch or commit SHA. Unused if the Git tag
    /// already exists. Default: the repository's default branch
    /// (usually `main`).
    pub fn target_commitish(
        mut self,
        target_commitish: &'target_commitish (impl AsRef<str> + ?Sized),
    ) -> Self {
        self.target_commitish = Some(target_commitish.as_ref());
        self
    }

    /// The name of the release.
    pub fn name(mut self, name: &'name (impl AsRef<str> + ?Sized)) -> Self {
        self.name = Some(name.as_ref());
        self
    }

    /// Text describing the contents of the tag.
    pub fn body(mut self, body: &'body (impl AsRef<str> + ?Sized)) -> Self {
        self.body = Some(body.as_ref());
        self
    }

    /// Whether to set the release as a "draft" release or not.
    pub fn draft(mut self, draft: impl Into<bool>) -> Self {
        self.draft = Some(draft.into());
        self
    }

    /// Whether to set the release as a "prerelease" or not.
    pub fn prerelease(mut self, prerelease: impl Into<bool>) -> Self {
        self.prerelease = Some(prerelease.into());
        self
    }

    /// Specifies whether this release should be set as the latest release for the repository.
    /// Drafts and prereleases cannot be set as latest.
    /// Defaults to [`MakeLatest::True`] for newly published releases.
    /// [`MakeLatest::Legacy`] specifies that the latest release should be determined based on the release creation date and higher semantic version.
    pub fn make_latest(mut self, make_latest: MakeLatest) -> Self {
        self.make_latest = Some(make_latest);
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> crate::Result<crate::models::repos::Release> {
        let route = format!(
            "/repos/{owner}/{repo}/releases",
            owner = self.handler.parent.owner,
            repo = self.handler.parent.repo
        );
        self.handler.parent.crab.post(route, Some(&self)).await
    }
}

/// A builder pattern struct for updating releases.
///
/// created by [`ReleasesHandler::update`].
#[derive(serde::Serialize)]
pub struct UpdateReleaseBuilder<'octo, 'repos, 'handler, 'tag_name, 'target_commitish, 'name, 'body>
{
    #[serde(skip)]
    handler: &'handler ReleasesHandler<'octo, 'repos>,
    release_id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag_name: Option<&'tag_name str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_commitish: Option<&'target_commitish str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'name str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<&'body str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    draft: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prerelease: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    make_latest: Option<MakeLatest>,
}

impl<'octo, 'repos, 'handler, 'tag_name, 'target_commitish, 'name, 'body>
    UpdateReleaseBuilder<'octo, 'repos, 'handler, 'tag_name, 'target_commitish, 'name, 'body>
{
    pub(crate) fn new(handler: &'handler ReleasesHandler<'octo, 'repos>, release_id: u64) -> Self {
        Self {
            handler,
            release_id,
            tag_name: None,
            target_commitish: None,
            name: None,
            body: None,
            draft: None,
            prerelease: None,
            make_latest: None,
        }
    }

    /// The release tag name.
    pub fn tag_name(mut self, tag_name: &'tag_name (impl AsRef<str> + ?Sized)) -> Self {
        self.tag_name = Some(tag_name.as_ref());
        self
    }

    /// Specifies the commitish value that determines where the Git tag is
    /// created from. Can be any branch or commit SHA. Unused if the Git tag
    /// already exists. Default: the repository's default branch
    /// (usually `main`).
    pub fn target_commitish(
        mut self,
        target_commitish: &'target_commitish (impl AsRef<str> + ?Sized),
    ) -> Self {
        self.target_commitish = Some(target_commitish.as_ref());
        self
    }

    /// The name of the release.
    pub fn name(mut self, name: &'name (impl AsRef<str> + ?Sized)) -> Self {
        self.name = Some(name.as_ref());
        self
    }

    /// Text describing the contents of the tag.
    pub fn body(mut self, body: &'body (impl AsRef<str> + ?Sized)) -> Self {
        self.body = Some(body.as_ref());
        self
    }

    /// Whether to set the release as a "draft" release or not.
    pub fn draft(mut self, draft: impl Into<bool>) -> Self {
        self.draft = Some(draft.into());
        self
    }

    /// Whether to set the release as a "prerelease" or not.
    pub fn prerelease(mut self, prerelease: impl Into<bool>) -> Self {
        self.prerelease = Some(prerelease.into());
        self
    }

    /// Specifies whether this release should be set as the latest release for the repository.
    /// Drafts and prereleases cannot be set as latest.
    /// [`MakeLatest::Legacy`] specifies that the latest release should be determined based on the release creation date and higher semantic version.
    pub fn make_latest(mut self, make_latest: MakeLatest) -> Self {
        self.make_latest = Some(make_latest);
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> crate::Result<crate::models::repos::Release> {
        let route = format!(
            "/repos/{owner}/{repo}/releases/{release_id}",
            owner = self.handler.parent.owner,
            repo = self.handler.parent.repo,
            release_id = self.release_id,
        );
        self.handler.parent.crab.patch(route, Some(&self)).await
    }
}

/// A builder pattern struct for updating releases.
///
/// created by [`ReleasesHandler::generate_release_notes`].
#[derive(serde::Serialize)]
pub struct GenerateReleaseNotesBuilder<
    'octo,
    'repos,
    'handler,
    'tag_name,
    'previous_tag_name,
    'target_commitish,
    'configuration_file_path,
> {
    #[serde(skip)]
    handler: &'handler ReleasesHandler<'octo, 'repos>,
    tag_name: &'tag_name str,
    #[serde(skip_serializing_if = "Option::is_none")]
    previous_tag_name: Option<&'previous_tag_name str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_commitish: Option<&'target_commitish str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    configuration_file_path: Option<&'configuration_file_path str>,
}

impl<
        'octo,
        'repos,
        'handler,
        'tag_name,
        'previous_tag_name,
        'target_commitish,
        'configuration_file_path,
    >
    GenerateReleaseNotesBuilder<
        'octo,
        'repos,
        'handler,
        'tag_name,
        'previous_tag_name,
        'target_commitish,
        'configuration_file_path,
    >
{
    pub(crate) fn new(
        handler: &'handler ReleasesHandler<'octo, 'repos>,
        tag_name: &'tag_name str,
    ) -> Self {
        Self {
            handler,
            tag_name,
            previous_tag_name: None,
            target_commitish: None,
            configuration_file_path: None,
        }
    }

    /// The tag which is used as a starting point for the release notes.
    pub fn previous_tag_name(
        mut self,
        previous_tag_name: &'previous_tag_name (impl AsRef<str> + ?Sized),
    ) -> Self {
        self.previous_tag_name = Some(previous_tag_name.as_ref());
        self
    }

    /// Specifies the commitish value that determines where the Git tag is
    /// created from. Can be any branch or commit SHA.
    /// Unused if the Git [`GenerateReleaseNotesBuilder::tag_name`] exists.
    pub fn target_commitish(
        mut self,
        target_commitish: &'target_commitish (impl AsRef<str> + ?Sized),
    ) -> Self {
        self.target_commitish = Some(target_commitish.as_ref());
        self
    }

    /// A file path within the repository which contains the configuration settings
    /// for generating release notes.
    pub fn configuration_file_path(
        mut self,
        configuration_file_path: &'configuration_file_path (impl AsRef<str> + ?Sized),
    ) -> Self {
        self.configuration_file_path = Some(configuration_file_path.as_ref());
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> crate::Result<crate::models::repos::ReleaseNotes> {
        let route = format!(
            "/repos/{owner}/{repo}/releases/generate-notes",
            owner = self.handler.parent.owner,
            repo = self.handler.parent.repo,
        );

        let result: Result<crate::models::repos::ReleaseNotes> =
            self.handler.parent.crab.post(route, Some(&self)).await;
        result
    }
}

// A builder pattern struct for listing release assets.
///
/// created by [`ReleasesHandler::assets`]
#[derive(serde::Serialize)]
pub struct ListReleaseAssetsBuilder<'octo, 'r1, 'r2> {
    #[serde(skip)]
    handler: &'r2 ReleasesHandler<'octo, 'r1>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r1, 'r2> ListReleaseAssetsBuilder<'octo, 'r1, 'r2> {
    pub(crate) fn new(handler: &'r2 ReleasesHandler<'octo, 'r1>) -> Self {
        Self {
            handler,
            per_page: None,
            page: None,
        }
    }

    /// Results per page (max 100).
    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.per_page = Some(per_page.into());
        self
    }

    /// Page number of the results to fetch.
    pub fn page(mut self, page: impl Into<u32>) -> Self {
        self.page = Some(page.into());
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> crate::Result<crate::Page<crate::models::repos::Asset>> {
        let route = format!(
            "/repos/{owner}/{repo}/releases/assets",
            owner = self.handler.parent.owner,
            repo = self.handler.parent.repo
        );
        self.handler.parent.crab.get(route, Some(&self)).await
    }
}

/// A builder pattern struct for updating release assets.
///
/// created by [`ReleasesHandler::upload_asset`].
pub struct UploadAssetBuilder<'octo, 'repos, 'handler, 'name, 'label> {
    handler: &'handler ReleasesHandler<'octo, 'repos>,
    release_id: u64,
    name: &'name str,
    body: Bytes,
    label: Option<&'label str>,
}

impl<'octo, 'repos, 'handler, 'name, 'label>
    UploadAssetBuilder<'octo, 'repos, 'handler, 'name, 'label>
{
    pub(crate) fn new(
        handler: &'handler ReleasesHandler<'octo, 'repos>,
        release_id: u64,
        name: &'name str,
        body: Bytes,
    ) -> Self {
        Self {
            handler,
            release_id,
            name,
            body,
            label: None,
        }
    }

    /// The asset label
    pub fn label(mut self, label: &'label (impl AsRef<str> + ?Sized)) -> Self {
        self.label = Some(label.as_ref());
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> crate::Result<Asset> {
        // the url could be constructed without fetching the release, but if the user has no access to the release
        // then he will not have access to upload to it.
        let release = self.handler.get(self.release_id).await?;

        // Documentation tells us to take the `upload_url`, but `upload_url` is just `assets_url` with `{?name,label}`.
        let mut url = release.assets_url.clone();
        url.query_pairs_mut().clear().append_pair("name", self.name);
        if let Some(label) = self.label {
            url.query_pairs_mut().append_pair("label", label);
        }
        let request = Builder::new()
            .method(http::Method::POST)
            .uri(url.to_string())
            .header(http::header::ACCEPT, "application/octet-stream")
            .body(OctoBody::from(self.body))
            .context(HttpSnafu)?;
        let response = self.handler.parent.crab.execute(request).await?;
        Asset::from_response(crate::map_github_error(response).await?).await
    }
}
