use crate::domain::user::User;

pub fn execute(
    user: &User,
) -> Result<String, String> {

    // versão temporária estável (sem refresh ainda)
    let token = format!("token-{}", user.id);

    Ok(token)
}
