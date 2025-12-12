use git2::{IndexAddOption, Repository};

pub trait RepoExt {
    fn create_initial_commit(&self) -> crate::Result<()>;
    fn create_commit(&self, message: impl AsRef<str>) -> crate::Result<()>;
    fn add_files(&self, paths: impl IntoIterator<Item = impl AsRef<str>>) -> crate::Result<()>;
}

impl RepoExt for Repository {
    fn create_initial_commit(&self) -> crate::Result<()> {
        let signature = self.signature()?;
        let oid = self.index()?.write_tree()?;
        let tree = self.find_tree(oid)?;
        self.commit(
            Some("HEAD"),
            &signature,
            &signature,
            "Initial commit",
            &tree,
            &[],
        )?;
        Ok(())
    }

    fn create_commit(&self, message: impl AsRef<str>) -> crate::Result<()> {
        let mut index = self.index()?;
        let oid = index.write_tree()?;
        let signature = self.signature()?;
        let parent_commit = self.head()?.peel_to_commit()?;
        let tree = self.find_tree(oid)?;
        self.commit(
            Some("HEAD"),
            &signature,
            &signature,
            message.as_ref(),
            &tree,
            &[&parent_commit],
        )?;
        Ok(())
    }

    fn add_files(&self, paths: impl IntoIterator<Item = impl AsRef<str>>) -> crate::Result<()> {
        let mut index = self.index()?;
        index.add_all(
            paths.into_iter().map(|v| v.as_ref().to_string()),
            IndexAddOption::DEFAULT,
            None,
        )?;
        index.write()?;
        Ok(())
    }
}
