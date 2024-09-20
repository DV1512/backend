use crate::auth::oauth::basic::Scopes;


#[derive(Debug, PartialEq, Clone)]
pub enum GithubScope {
    NoScope,
    Repo,
    RepoStatus,
    RepoDeployment,
    PublicRepo,
    RepoInvite,
    SecurityEvents,
    AdminRepoHook,
    WriteRepoHook,
    ReadRepoHook,
    AdminOrg,
    WriteOrg,
    ReadOrg,
    AdminPublicKey,
    WritePublicKey,
    ReadPublicKey,
    AdminOrgHook,
    Gist,
    Notifications,
    User,
    ReadUser,
    UserEmail,
    UserFollow,
    Project,
    ReadProject,
    DeleteRepo,
    WritePackages,
    ReadPackages,
    DeletePackages,
    AdminGpgKey,
    WriteGpgKey,
    ReadGpgKey,
    CodesSpace,
    Workflow,
}

impl Default for GithubScope {
    fn default() -> Self {
        GithubScope::NoScope
    }
}

impl From<GithubScope> for String {
    fn from(scope: GithubScope) -> Self {
        match scope{
            GithubScope::NoScope => "(no scope)".to_string(),
            GithubScope::Repo => "repo".to_string(),
            GithubScope::RepoStatus => "repo:status".to_string(),
            GithubScope::RepoDeployment => "repo_deployment".to_string(),
            GithubScope::PublicRepo => "public_repo".to_string(),
            GithubScope::RepoInvite => "repo:invite".to_string(),
            GithubScope::SecurityEvents => "security_events".to_string(),
            GithubScope::AdminRepoHook => "admin:repo_hook".to_string(),
            GithubScope::WriteRepoHook => "write:repo_hook".to_string(),
            GithubScope::ReadRepoHook => "read:repo_hook".to_string(),
            GithubScope::AdminOrg => "admin:org".to_string(),
            GithubScope::WriteOrg => "write:org".to_string(),
            GithubScope::ReadOrg => "read:org".to_string(),
            GithubScope::AdminPublicKey => "admin:public_key".to_string(),
            GithubScope::WritePublicKey => "write:public_key".to_string(),
            GithubScope::ReadPublicKey => "read:public_key".to_string(),
            GithubScope::AdminOrgHook => "admin:org_hook".to_string(),
            GithubScope::Gist => "gist".to_string(),
            GithubScope::Notifications => "notifications".to_string(),
            GithubScope::User => "user".to_string(),
            GithubScope::ReadUser => "read:user".to_string(),
            GithubScope::UserEmail => "user:email".to_string(),
            GithubScope::UserFollow => "user:follow".to_string(),
            GithubScope::Project => "project".to_string(),
            GithubScope::ReadProject => "read:project".to_string(),
            GithubScope::DeleteRepo => "delete_repo".to_string(),
            GithubScope::WritePackages => "write:packages".to_string(),
            GithubScope::ReadPackages => "read:packages".to_string(),
            GithubScope::DeletePackages => "delete:packages".to_string(),
            GithubScope::AdminGpgKey => "admin:gpg_key".to_string(),
            GithubScope::WriteGpgKey => "write:gpg_key".to_string(),
            GithubScope::ReadGpgKey => "read:gpg_key".to_string(),
            GithubScope::CodesSpace => "codespace".to_string(),
            GithubScope::Workflow => "workflow".to_string(),
        }
    }

}

impl From<&GithubScope> for String {
    fn from(scope: &GithubScope) -> Self {
        match scope{
            GithubScope::NoScope => "(no scope)".to_string(),
            GithubScope::Repo => "repo".to_string(),
            GithubScope::RepoStatus => "repo:status".to_string(),
            GithubScope::RepoDeployment => "repo_deployment".to_string(),
            GithubScope::PublicRepo => "public_repo".to_string(),
            GithubScope::RepoInvite => "repo:invite".to_string(),
            GithubScope::SecurityEvents => "security_events".to_string(),
            GithubScope::AdminRepoHook => "admin:repo_hook".to_string(),
            GithubScope::WriteRepoHook => "write:repo_hook".to_string(),
            GithubScope::ReadRepoHook => "read:repo_hook".to_string(),
            GithubScope::AdminOrg => "admin:org".to_string(),
            GithubScope::WriteOrg => "write:org".to_string(),
            GithubScope::ReadOrg => "read:org".to_string(),
            GithubScope::AdminPublicKey => "admin:public_key".to_string(),
            GithubScope::WritePublicKey => "write:public_key".to_string(),
            GithubScope::ReadPublicKey => "read:public_key".to_string(),
            GithubScope::AdminOrgHook => "admin:org_hook".to_string(),
            GithubScope::Gist => "gist".to_string(),
            GithubScope::Notifications => "notifications".to_string(),
            GithubScope::User => "user".to_string(),
            GithubScope::ReadUser => "read:user".to_string(),
            GithubScope::UserEmail => "user:email".to_string(),
            GithubScope::UserFollow => "user:follow".to_string(),
            GithubScope::Project => "project".to_string(),
            GithubScope::ReadProject => "read:project".to_string(),
            GithubScope::DeleteRepo => "delete_repo".to_string(),
            GithubScope::WritePackages => "write:packages".to_string(),
            GithubScope::ReadPackages => "read:packages".to_string(),
            GithubScope::DeletePackages => "delete:packages".to_string(),
            GithubScope::AdminGpgKey => "admin:gpg_key".to_string(),
            GithubScope::WriteGpgKey => "write:gpg_key".to_string(),
            GithubScope::ReadGpgKey => "read:gpg_key".to_string(),
            GithubScope::CodesSpace => "codespace".to_string(),
            GithubScope::Workflow => "workflow".to_string(),
        }
    }

}

impl From<GithubScope> for &str{
    fn from(scope: GithubScope) -> Self {
        match scope{
            GithubScope::NoScope => "(no scope)",
            GithubScope::Repo => "repo",
            GithubScope::RepoStatus => "repo:status",
            GithubScope::RepoDeployment => "repo_deployment",
            GithubScope::PublicRepo => "public_repo",
            GithubScope::RepoInvite => "repo:invite",
            GithubScope::SecurityEvents => "security_events",
            GithubScope::AdminRepoHook => "admin:repo_hook",
            GithubScope::WriteRepoHook => "write:repo_hook",
            GithubScope::ReadRepoHook => "read:repo_hook",
            GithubScope::AdminOrg => "admin:org",
            GithubScope::WriteOrg => "write:org",
            GithubScope::ReadOrg => "read:org",
            GithubScope::AdminPublicKey => "admin:public_key",
            GithubScope::WritePublicKey => "write:public_key",
            GithubScope::ReadPublicKey => "read:public_key",
            GithubScope::AdminOrgHook => "admin:org_hook",
            GithubScope::Gist => "gist",
            GithubScope::Notifications => "notifications",
            GithubScope::User => "user",
            GithubScope::ReadUser => "read:user",
            GithubScope::UserEmail => "user:email",
            GithubScope::UserFollow => "user:follow",
            GithubScope::Project => "project",
            GithubScope::ReadProject => "read:project",
            GithubScope::DeleteRepo => "delete_repo",
            GithubScope::WritePackages => "write:packages",
            GithubScope::ReadPackages => "read:packages",
            GithubScope::DeletePackages => "delete:packages",
            GithubScope::AdminGpgKey => "admin:gpg_key",
            GithubScope::WriteGpgKey => "write:gpg_key",
            GithubScope::ReadGpgKey => "read:gpg_key",
            GithubScope::CodesSpace => "codespace",
            GithubScope::Workflow => "workflow",
        }
    }

}


impl From<&GithubScope> for &str{
    fn from(scope: &GithubScope) -> Self {
        match scope{
            GithubScope::NoScope => "(no scope)",
            GithubScope::Repo => "repo",
            GithubScope::RepoStatus => "repo:status",
            GithubScope::RepoDeployment => "repo_deployment",
            GithubScope::PublicRepo => "public_repo",
            GithubScope::RepoInvite => "repo:invite",
            GithubScope::SecurityEvents => "security_events",
            GithubScope::AdminRepoHook => "admin:repo_hook",
            GithubScope::WriteRepoHook => "write:repo_hook",
            GithubScope::ReadRepoHook => "read:repo_hook",
            GithubScope::AdminOrg => "admin:org",
            GithubScope::WriteOrg => "write:org",
            GithubScope::ReadOrg => "read:org",
            GithubScope::AdminPublicKey => "admin:public_key",
            GithubScope::WritePublicKey => "write:public_key",
            GithubScope::ReadPublicKey => "read:public_key",
            GithubScope::AdminOrgHook => "admin:org_hook",
            GithubScope::Gist => "gist",
            GithubScope::Notifications => "notifications",
            GithubScope::User => "user",
            GithubScope::ReadUser => "read:user",
            GithubScope::UserEmail => "user:email",
            GithubScope::UserFollow => "user:follow",
            GithubScope::Project => "project",
            GithubScope::ReadProject => "read:project",
            GithubScope::DeleteRepo => "delete_repo",
            GithubScope::WritePackages => "write:packages",
            GithubScope::ReadPackages => "read:packages",
            GithubScope::DeletePackages => "delete:packages",
            GithubScope::AdminGpgKey => "admin:gpg_key",
            GithubScope::WriteGpgKey => "write:gpg_key",
            GithubScope::ReadGpgKey => "read:gpg_key",
            GithubScope::CodesSpace => "codespace",
            GithubScope::Workflow => "workflow",
        }
    }
}


impl From<String> for GithubScope {
    fn from(scope: String) -> Self {
        match scope.as_str(){
            "(no scope)" => GithubScope::NoScope,
            "repo" => GithubScope::Repo,
            "repo:status" => GithubScope::RepoStatus,
            "repo_deployment" => GithubScope::RepoDeployment,
            "public_repo" => GithubScope::PublicRepo,
            "repo:invite" => GithubScope::RepoInvite,
            "security_events" => GithubScope::SecurityEvents,
            "admin:repo_hook" => GithubScope::AdminRepoHook,
            "write:repo_hook" => GithubScope::WriteRepoHook,
            "read:repo_hook" => GithubScope::ReadRepoHook,
            "admin:org" => GithubScope::AdminOrg,
            "Write:org" => GithubScope::WriteOrg,
            "read:org" => GithubScope::ReadOrg,
            "admin:public_key" => GithubScope::AdminPublicKey,
            "write:public_key" => GithubScope::WritePublicKey,
            "read:public_key" => GithubScope::ReadPublicKey,
            "admin:org_hook" => GithubScope::AdminOrgHook,
            "gist" => GithubScope::Gist,
            "notifications" => GithubScope::Notifications,
            "user" => GithubScope::User,
            "read:user" => GithubScope::ReadUser,
            "user:email" => GithubScope::UserEmail,
            "user:follow" => GithubScope::UserFollow,
            "project" => GithubScope::Project,
            "read:project" => GithubScope::ReadProject,
            "delete_repo" => GithubScope::DeleteRepo,
            "write:packages" => GithubScope::WritePackages,
            "read:packages" => GithubScope::ReadPackages,
            "delete:packages" => GithubScope::DeletePackages,
            "admin:gpg_key" => GithubScope::AdminGpgKey,
            "write:gpg_key" => GithubScope::WriteGpgKey,
            "read:gpg_key" => GithubScope::ReadGpgKey,
            "codes_space" => GithubScope::CodesSpace,
            "workflow" => GithubScope::Workflow,
            _ => GithubScope::NoScope,
        }
    }
}

impl From<&String> for GithubScope {
    fn from(scope: &String) -> Self {
        match scope.as_str(){
            "(no scope)" => GithubScope::NoScope,
            "repo" => GithubScope::Repo,
            "repo:status" => GithubScope::RepoStatus,
            "repo_deployment" => GithubScope::RepoDeployment,
            "public_repo" => GithubScope::PublicRepo,
            "repo:invite" => GithubScope::RepoInvite,
            "security_events" => GithubScope::SecurityEvents,
            "admin:repo_hook" => GithubScope::AdminRepoHook,
            "write:repo_hook" => GithubScope::WriteRepoHook,
            "read:repo_hook" => GithubScope::ReadRepoHook,
            "admin:org" => GithubScope::AdminOrg,
            "Write:org" => GithubScope::WriteOrg,
            "read:org" => GithubScope::ReadOrg,
            "admin:public_key" => GithubScope::AdminPublicKey,
            "write:public_key" => GithubScope::WritePublicKey,
            "read:public_key" => GithubScope::ReadPublicKey,
            "admin:org_hook" => GithubScope::AdminOrgHook,
            "gist" => GithubScope::Gist,
            "notifications" => GithubScope::Notifications,
            "user" => GithubScope::User,
            "read:user" => GithubScope::ReadUser,
            "user:email" => GithubScope::UserEmail,
            "user:follow" => GithubScope::UserFollow,
            "project" => GithubScope::Project,
            "read:project" => GithubScope::ReadProject,
            "delete_repo" => GithubScope::DeleteRepo,
            "write:packages" => GithubScope::WritePackages,
            "read:packages" => GithubScope::ReadPackages,
            "delete:packages" => GithubScope::DeletePackages,
            "admin:gpg_key" => GithubScope::AdminGpgKey,
            "write:gpg_key" => GithubScope::WriteGpgKey,
            "read:gpg_key" => GithubScope::ReadGpgKey,
            "codes_space" => GithubScope::CodesSpace,
            "workflow" => GithubScope::Workflow,
            _ => GithubScope::NoScope,

        }
    }

}

impl From<&str> for GithubScope {
    fn from(scope: &str) -> Self {
        match scope{
            "(no scope)" => GithubScope::NoScope,
            "repo" => GithubScope::Repo,
            "repo:status" => GithubScope::RepoStatus,
            "repo_deployment" => GithubScope::RepoDeployment,
            "public_repo" => GithubScope::PublicRepo,
            "repo:invite" => GithubScope::RepoInvite,
            "security_events" => GithubScope::SecurityEvents,
            "admin:repo_hook" => GithubScope::AdminRepoHook,
            "write:repo_hook" => GithubScope::WriteRepoHook,
            "read:repo_hook" => GithubScope::ReadRepoHook,
            "admin:org" => GithubScope::AdminOrg,
            "Write:org" => GithubScope::WriteOrg,
            "read:org" => GithubScope::ReadOrg,
            "admin:public_key" => GithubScope::AdminPublicKey,
            "write:public_key" => GithubScope::WritePublicKey,
            "read:public_key" => GithubScope::ReadPublicKey,
            "admin:org_hook" => GithubScope::AdminOrgHook,
            "gist" => GithubScope::Gist,
            "notifications" => GithubScope::Notifications,
            "user" => GithubScope::User,
            "read:user" => GithubScope::ReadUser,
            "user:email" => GithubScope::UserEmail,
            "user:follow" => GithubScope::UserFollow,
            "project" => GithubScope::Project,
            "read:project" => GithubScope::ReadProject,
            "delete_repo" => GithubScope::DeleteRepo,
            "write:packages" => GithubScope::WritePackages,
            "read:packages" => GithubScope::ReadPackages,
            "delete:packages" => GithubScope::DeletePackages,
            "admin:gpg_key" => GithubScope::AdminGpgKey,
            "write:gpg_key" => GithubScope::WriteGpgKey,
            "read:gpg_key" => GithubScope::ReadGpgKey,
            "codes_space" => GithubScope::CodesSpace,
            "workflow" => GithubScope::Workflow,
            _ => GithubScope::NoScope,

        }
    }

}

impl From<&str> for &GithubScope {
    fn from(scope: &str) -> Self {
        match scope {
            "(no scope)" => &GithubScope::NoScope,
            "repo" => &GithubScope::Repo,
            "repo:status" => &GithubScope::RepoStatus,
            "repo_deployment" => &GithubScope::RepoDeployment,
            "public_repo" => &GithubScope::PublicRepo,
            "repo:invite" => &GithubScope::RepoInvite,
            "security_events" => &GithubScope::SecurityEvents,
            "admin:repo_hook" => &GithubScope::AdminRepoHook,
            "write:repo_hook" => &GithubScope::WriteRepoHook,
            "read:repo_hook" => &GithubScope::ReadRepoHook,
            "admin:org" => &GithubScope::AdminOrg,
            "Write:org" => &GithubScope::WriteOrg,
            "read:org" => &GithubScope::ReadOrg,
            "admin:public_key" => &GithubScope::AdminPublicKey,
            "write:public_key" => &GithubScope::WritePublicKey,
            "read:public_key" => &GithubScope::ReadPublicKey,
            "admin:org_hook" => &GithubScope::AdminOrgHook,
            "gist" => &GithubScope::Gist,
            "notifications" => &GithubScope::Notifications,
            "user" => &GithubScope::User,
            "read:user" => &GithubScope::ReadUser,
            "user:email" => &GithubScope::UserEmail,
            "user:follow" => &GithubScope::UserFollow,
            "project" => &GithubScope::Project,
            "read:project" => &GithubScope::ReadProject,
            "delete_repo" => &GithubScope::DeleteRepo,
            "write:packages" => &GithubScope::WritePackages,
            "read:packages" => &GithubScope::ReadPackages,
            "delete:packages" => &GithubScope::DeletePackages,
            "admin:gpg_key" => &GithubScope::AdminGpgKey,
            "write:gpg_key" => &GithubScope::WriteGpgKey,
            "read:gpg_key" => &GithubScope::ReadGpgKey,
            "codes_space" => &GithubScope::CodesSpace,
            "workflow" => &GithubScope::Workflow,
            _ => &GithubScope::NoScope,

        }
    }

}
impl From<String> for &GithubScope {
    fn from(scope: String) -> Self {
        match scope.as_str() {
            "(no scope)" => &GithubScope::NoScope,
            "repo" => &GithubScope::Repo,
            "repo:status" => &GithubScope::RepoStatus,
            "repo_deployment" => &GithubScope::RepoDeployment,
            "public_repo" => &GithubScope::PublicRepo,
            "repo:invite" => &GithubScope::RepoInvite,
            "security_events" => &GithubScope::SecurityEvents,
            "admin:repo_hook" => &GithubScope::AdminRepoHook,
            "write:repo_hook" => &GithubScope::WriteRepoHook,
            "read:repo_hook" => &GithubScope::ReadRepoHook,
            "admin:org" => &GithubScope::AdminOrg,
            "Write:org" => &GithubScope::WriteOrg,
            "read:org" => &GithubScope::ReadOrg,
            "admin:public_key" => &GithubScope::AdminPublicKey,
            "write:public_key" => &GithubScope::WritePublicKey,
            "read:public_key" => &GithubScope::ReadPublicKey,
            "admin:org_hook" => &GithubScope::AdminOrgHook,
            "gist" => &GithubScope::Gist,
            "notifications" => &GithubScope::Notifications,
            "user" => &GithubScope::User,
            "read:user" => &GithubScope::ReadUser,
            "user:email" => &GithubScope::UserEmail,
            "user:follow" => &GithubScope::UserFollow,
            "project" => &GithubScope::Project,
            "read:project" => &GithubScope::ReadProject,
            "delete_repo" => &GithubScope::DeleteRepo,
            "write:packages" => &GithubScope::WritePackages,
            "read:packages" => &GithubScope::ReadPackages,
            "delete:packages" => &GithubScope::DeletePackages,
            "admin:gpg_key" => &GithubScope::AdminGpgKey,
            "write:gpg_key" => &GithubScope::WriteGpgKey,
            "read:gpg_key" => &GithubScope::ReadGpgKey,
            "codes_space" => &GithubScope::CodesSpace,
            "workflow" => &GithubScope::Workflow,
            _ => &GithubScope::NoScope,

        }
    }

}



#[derive(Default)]
pub struct GithubScopes{
    scopes: Vec<GithubScope>,
}


impl From<Vec<GithubScope>> for GithubScopes {
    fn from(scopes: Vec<GithubScope>) -> Self {
        GithubScopes { scopes }
    }
}

impl From<GithubScopes> for Vec<String>{
    fn from(scopes: GithubScopes) -> Self {
        scopes.scopes.iter().map(|s| s.into()).collect()
    }
}

impl Scopes<GithubScope> for GithubScopes {
    fn add_scope(mut self, scope: GithubScope) -> Self {
        self.scopes.push(scope);
        self
    }
    fn remove_scope(mut self, scope: GithubScope) -> Self {
        self.scopes.retain(|s| s != &scope);
        self
    }
    fn scopes(&self) -> Vec<&str> {
        self.scopes.iter().map(|s| s.into()).collect()
    }
}

