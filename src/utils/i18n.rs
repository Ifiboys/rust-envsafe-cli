#![allow(dead_code)]

pub struct Translations {
    pub login: LoginTranslations,
    pub logout: LogoutTranslations,
    pub pull: PullTranslations,
    pub push: PushTranslations,
    pub list: ListTranslations,
}

pub struct LoginTranslations {
    pub success: &'static str,
    pub failed: &'static str,
}

pub struct LogoutTranslations {
    pub success: &'static str,
}

pub struct PullTranslations {
    pub success: &'static str,
    pub failed: &'static str,
}

pub struct PushTranslations {
    pub success: &'static str,
    pub failed: &'static str,
}

pub struct ListTranslations {
    pub no_projects: &'static str,
}

pub fn get_translations(lang: &str) -> Translations {
    match lang {
        "fr" => Translations {
            login: LoginTranslations {
                success: "Connexion réussie",
                failed: "Échec de la connexion",
            },
            logout: LogoutTranslations {
                success: "Déconnexion réussie",
            },
            pull: PullTranslations {
                success: "Variables téléchargées avec succès",
                failed: "Échec du téléchargement",
            },
            push: PushTranslations {
                success: "Variables envoyées avec succès",
                failed: "Échec de l'envoi",
            },
            list: ListTranslations {
                no_projects: "Aucun projet trouvé",
            },
        },
        _ => Translations {
            login: LoginTranslations {
                success: "Successfully logged in",
                failed: "Login failed",
            },
            logout: LogoutTranslations {
                success: "Successfully logged out",
            },
            pull: PullTranslations {
                success: "Variables pulled successfully",
                failed: "Failed to pull variables",
            },
            push: PushTranslations {
                success: "Variables pushed successfully",
                failed: "Failed to push variables",
            },
            list: ListTranslations {
                no_projects: "No projects found",
            },
        },
    }
}
