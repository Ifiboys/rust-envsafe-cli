#![allow(dead_code)]

pub struct Translations {
    pub common: CommonTranslations,
    pub error: ErrorTranslations,
    pub init: InitTranslations,
    pub login: LoginTranslations,
    pub logout: LogoutTranslations,
    pub pull: PullTranslations,
    pub push: PushTranslations,
    pub list: ListTranslations,
    pub whoami: WhoamiTranslations,
    pub watch: WatchTranslations,
    pub run: RunTranslations,
}

pub struct CommonTranslations {
    pub select_workspace: &'static str,
    pub select_project: &'static str,
    pub no_workspaces: &'static str,
    pub no_projects: &'static str,
    pub success: &'static str,
    pub loading: &'static str,
}

pub struct ErrorTranslations {
    pub unauthorized: &'static str,
    pub unknown: &'static str,
    pub not_found: &'static str,
}

pub struct InitTranslations {
    pub title: &'static str,
    pub success: &'static str,
    pub already_initialized: &'static str,
    pub creating_config: &'static str,
}

pub struct LoginTranslations {
    pub success: &'static str,
    pub failed: &'static str,
    pub opening_browser: &'static str,
    pub enter_token: &'static str,
}

pub struct WhoamiTranslations {
    pub not_logged_in: &'static str,
    pub logged_as: &'static str,
    pub title: &'static str,
    pub name: &'static str,
    pub email: &'static str,
    pub last_login: &'static str,
}

pub struct WatchTranslations {
    pub title: &'static str,
    pub sync_start: &'static str,
    pub remote_to_local: &'static str,
    pub local_to_remote: &'static str,
    pub press_ctrl_c: &'static str,
    pub start: &'static str,
    pub connected: &'static str,
    pub disconnected: &'static str,
    pub update_received: &'static str,
}

pub struct RunTranslations {
    pub executing: &'static str,
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
            common: CommonTranslations {
                select_workspace: "Sélectionnez un espace de travail",
                select_project: "Sélectionnez un projet",
                no_workspaces: "Aucun espace de travail trouvé",
                no_projects: "Aucun projet trouvé dans cet espace",
                success: "Succès",
                loading: "Chargement...",
            },
            error: ErrorTranslations {
                unauthorized: "Non autorisé. Veuillez vous connecter.",
                unknown: "Une erreur inconnue est survenue",
                not_found: "Non trouvé",
            },
            init: InitTranslations {
                title: "🚀 Initialisation du projet",
                success: "Projet initialisé avec succès",
                already_initialized: "Ce projet est déjà initialisé",
                creating_config: "Création de la configuration local...",
            },
            login: LoginTranslations {
                success: "Connexion réussie",
                failed: "Échec de la connexion",
                opening_browser: "Ouverture du navigateur...",
                enter_token: "Entrez votre token API :",
            },
            whoami: WhoamiTranslations {
                not_logged_in: "Vous n'êtes pas connecté",
                logged_as: "Connecté en tant que :",
                title: "👤 Utilisateur Actuel",
                name: "  Nom : {}",
                email: "  Email : {}",
                last_login: "  Dernière connexion : {}",
            },
            watch: WatchTranslations {
                title: "👁️  Mode Surveillance EnvSafe",
                sync_start: "🔄 Démarrage de la synchronisation bidirectionnelle...",
                remote_to_local: "  - Changements distants → Fichier local",
                local_to_remote: "  - Fichier local → Distant",
                press_ctrl_c: "Appuyez sur Ctrl+C pour arrêter",
                start: "Démarrage du mode surveillance...",
                connected: "Connecté au serveur de mise à jour",
                disconnected: "Déconnecté du serveur",
                update_received: "Mise à jour reçue",
            },
            run: RunTranslations {
                executing: "Exécution de la commande avec les variables injectées...",
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
            common: CommonTranslations {
                select_workspace: "Select a workspace",
                select_project: "Select a project",
                no_workspaces: "No workspaces found",
                no_projects: "No projects found in this workspace",
                success: "Success",
                loading: "Loading...",
            },
            error: ErrorTranslations {
                unauthorized: "Unauthorized. Please login.",
                unknown: "An unknown error occurred",
                not_found: "Not found",
            },
            init: InitTranslations {
                title: "🚀 Initialize Project",
                success: "Project initialized successfully",
                already_initialized: "Project is already initialized",
                creating_config: "Creating local configuration...",
            },
            login: LoginTranslations {
                success: "Successfully logged in",
                failed: "Login failed",
                opening_browser: "Opening browser...",
                enter_token: "Enter your API token:",
            },
            whoami: WhoamiTranslations {
                not_logged_in: "You are not logged in",
                logged_as: "Logged in as:",
                title: "👤 Current User",
                name: "  Name: {}",
                email: "  Email: {}",
                last_login: "  Last login: {}",
            },
            watch: WatchTranslations {
                title: "👁️  EnvSafe Watch Mode",
                sync_start: "🔄 Starting bidirectional sync...",
                remote_to_local: "  - Remote changes → Local file",
                local_to_remote: "  - Local file → Remote",
                press_ctrl_c: "Press Ctrl+C to stop watching",
                start: "Starting watch mode...",
                connected: "Connected to update server",
                disconnected: "Disconnected from server",
                update_received: "Update received",
            },
            run: RunTranslations {
                executing: "Executing command with injected variables...",
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
