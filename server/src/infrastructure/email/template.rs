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

// 3. Generic event notification using the base template.html
#[derive(Template)]
#[template(path = "template.html")]
pub struct EventNotificationTemplate<'a> {
    pub email_title: &'a str,
    pub heading: &'a str,
    pub intro_text: &'a str,
    pub detail_label: &'a str,
    pub detail_value: &'a str,
    pub closing_text: &'a str,
}
