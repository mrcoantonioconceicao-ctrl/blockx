use uuid::Uuid;

use crate::domain::authorization::role_permission::RolePermission;

/// Associa uma Permission a uma Role.
pub fn execute(
    role_id: Uuid,
    permission_id: Uuid,
) -> Result<RolePermission, String> {
    Ok(RolePermission::new(
        role_id,
        permission_id,
    ))
}
