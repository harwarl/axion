pub const NEW_PROJECT_DIR: &[&str] = &[
    "src",
    // API DIRECTORIES
    "src/api",
    "src/api/controllers",
    "src/api/dtos",
    "src/api/middlewares",
    // DOMAIN DIRECTORIES
    "src/domain",
    "src/domain/models",
    "src/domain/repositories",
    "src/domain/services",
    // INFRASTRUCTURE DIRECTORIES
    "src/infrastructure",
    "src/infrastructure/databases",
    "src/infrastructure/models",
    "src/infrastructure/services",
    // SERVICES DIRECTORIES
    "src/services",
    // TEST DIRECTORIES
    "src/tests",
];

pub const MAIN_FILES: &[&str] = &[
    "src/main.rs",
    "src/create_app.rs",
    "src/lib.rs",
    "src/state.rs",
    // API DIRECTORIES
    "src/api/mod.rs",
    "src/api/controllers/mod.rs",
    "src/api/dtos/mod.rs",
    "src/api/middlewares/mod.rs",
    // DOMAIN DIRECTORIES
    "src/domain/mod.rs",
    "src/domain/error.rs",
    "src/domain/constant.rs",
    "src/domain/models/mod.rs",
    "src/domain/repositories/mod.rs",
    "src/domain/services/mod.rs",
    // INFRASTRUCTURE DIRECTORIES
    "src/infrastructure/mod.rs",
    "src/infrastructure/error.rs",
    "src/infrastructure/schema.rs",
    "src/infrastructure/databases/mod.rs",
    "src/infrastructure/models/mod.rs",
    "src/infrastructure/services/mod.rs",
    // SERVICES DIRECTORIES
    "src/services/mod.rs",
    // TEST DIRECTORIES
    // "src/tests",
];
