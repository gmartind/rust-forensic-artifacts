
pub trait Artifact {
    fn get_artifact(&self) -> Result<String, String>;
}