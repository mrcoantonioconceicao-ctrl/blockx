use crate::domain::authorization::permission::Permission;

/// Caso de uso responsável pela criação de uma Permission.
pub fn execute(
    code: impl Into<String>,
    description: impl Into<String>,
) -> Result<Permission, String> {
    let code = code.into();

    if code.trim().is_empty() {
        return Err("permission code cannot be empty".to_string());
    }

    Ok(Permission::new(
        code,
        description,
    ))
}
