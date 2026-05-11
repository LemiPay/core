use askama::Template;

// 1. Template de Registro
#[derive(Template)]
#[template(path = "register.html")]
pub struct RegisterTemplate<'a> {
    pub user_name: &'a str,
    pub action_url: &'a str,
}

// 2. Template de Alerta de Login
#[derive(Template)]
#[template(path = "login_alert.html")]
pub struct LoginAlertTemplate<'a> {
    pub user_name: &'a str,
    pub time: &'a str,
}
