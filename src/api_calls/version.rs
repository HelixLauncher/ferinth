use super::check_id_slug;
use crate::{
    request::API_URL_BASE, structures::version::*, url_join_ext::UrlJoinExt, Ferinth,
    Result,
};

impl Ferinth {
    /// Get the versions of project with ID `project_id`
    ///
    /// Example:
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), ferinth::Error> {
    /// # let modrinth = ferinth::Ferinth::default();
    /// let sodium_versions = modrinth.list_versions("AANobbMI").await?;
    /// assert!(sodium_versions[0].project_id == "AANobbMI");
    /// # Ok(()) }
    /// ```
    pub async fn list_versions(&self, project_id: &str) -> Result<Vec<Version>> {
        check_id_slug(project_id)?;
        self.get(API_URL_BASE.join_all(vec!["project", project_id, "version"]))
            .await
    }

    /// Get the versions of project with ID `project_id` with filters
    ///
    /// `loaders`: The types of loaders to filter for
    /// `game_versions`: The game versions to filter for
    /// `featured`: Filter for featured or non-featured versions only
    ///
    /// Example:
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), ferinth::Error> {
    /// # let modrinth = ferinth::Ferinth::default();
    /// let sodium_forge_versions = modrinth.list_versions_filtered("AANobbMI", Some(&["forge"]), None, None).await?;
    /// assert!(sodium_forge_versions.is_empty());
    /// # Ok(()) }
    /// ```
    pub async fn list_versions_filtered(
        &self,
        project_id: &str,
        loaders: Option<&[&str]>,
        game_versions: Option<&[&str]>,
        featured: Option<bool>,
    ) -> Result<Vec<Version>> {
        check_id_slug(project_id)?;
        let mut query = Vec::new();
        if let Some(loaders) = loaders {
            query.push(("loaders", serde_json::to_string(loaders)?));
        }
        if let Some(game_versions) = game_versions {
            query.push(("game_versions", serde_json::to_string(game_versions)?));
        }
        if let Some(featured) = featured {
            query.push(("featured", serde_json::to_string(&featured)?));
        }
        let query = query
            .into_iter()
            .map(|this| (this.0, this.1))
            .collect::<Vec<_>>();
        self.get_with_query(
            API_URL_BASE.join_all(vec!["project", project_id, "version"]),
            &query,
        )
        .await
    }

    /// Get version with ID `version_id`
    ///
    /// Example:
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), ferinth::Error> {
    /// # let modrinth = ferinth::Ferinth::default();
    /// let sodium_version = modrinth.get_version("xuWxRZPd").await?;
    /// assert!(sodium_version.project_id == "AANobbMI");
    /// # Ok(()) }
    /// ```
    pub async fn get_version(&self, version_id: &str) -> Result<Version> {
        check_id_slug(version_id)?;
        self.get(API_URL_BASE.join_all(vec!["version", version_id]))
            .await
    }

    /// Get multiple versions with IDs `version_ids`
    ///
    /// Example:
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), ferinth::Error> {
    /// # let modrinth = ferinth::Ferinth::default();
    /// let versions = modrinth.get_multiple_versions(&[
    ///     "sxWTUZpD",
    ///     "mgPpe4NY",
    /// ]).await?;
    /// for version in versions {
    ///     assert!(version.project_id == "of7wIinq");
    /// }
    /// # Ok(()) }
    /// ```
    pub async fn get_multiple_versions(&self, version_ids: &[&str]) -> Result<Vec<Version>> {
        for versions_id in version_ids {
            check_id_slug(versions_id)?;
        }
        self.get_with_query(
            API_URL_BASE.join_all(vec!["versions"]),
            &[("ids", &serde_json::to_string(version_ids)?)],
        )
        .await
    }
}
