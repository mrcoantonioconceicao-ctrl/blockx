use crate::domain::authorization::role::Role;

/// Caso de uso responsável pela criação de uma Role.
///
/// Nesta primeira versão apenas instancia a entidade.
/// A persistência será adicionada quando criarmos os
/// repositórios PostgreSQL/SQLx.
pub fn execute(
    name: impl Into<String>,
    description: impl Into<String>,
) -> Result<Role, String> {
    let name = name.into();

    if name.trim().is_empty() {
        return Err("role name cannot be empty".to_string());
    }

    Ok(Role::new(
        name,
        description,
    ))
}
