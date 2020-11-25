use crate::{error::AuthError, session::Confirmation, vars};
use lettre::{smtp, ClientSecurity, ClientTlsParameters, SmtpClient, Transport};
use lettre_email::EmailBuilder;
use native_tls::{Protocol, TlsConnector};

pub fn send_confirmation_mail(confirmation: &Confirmation) -> Result<(), AuthError> {
    let domain_url = vars::domain_url();
    let smtp_host = vars::smtp_host();
    let expires = confirmation
        .expires_at
        .format("%I:%M %p %A, %-d %B, %C%y")
        .to_string();
    let html_text = format!(
        "Please click on the link below to complete registration. <br/>
       <a href=\"{domain}/register?id={id}&email={email}\">Complete registration</a>",
        domain = domain_url,
        id = confirmation.id,
        email = confirmation.email
    );
    let plain_text = format!(
        "Please visit the link below to complete registration:\n
      {domain}/register.html?id={id}&email={email}\n
      This link expires on {expires}.",
        domain = domain_url,
        id = confirmation.id,
        email = confirmation.email,
        expires = expires
    );
    let email = EmailBuilder::new()
        .to(confirmation.email.clone())
        .from(("noreply@auth-service.com", vars::smtp_sender_name()))
        .subject("Complete registration")
        .text(plain_text)
        .html(html_text)
        .build()
        .unwrap();
    let tls_connect = TlsConnector::builder()
        .min_protocol_version(Some(Protocol::Tlsv10))
        .build()
        .unwrap();
    let tls_parameters = ClientTlsParameters::new(smtp_host.clone(), tls_connect);
    let mut mailer = SmtpClient::new(
        (smtp_host.as_str(), vars::smtp_port()),
        ClientSecurity::Required(tls_parameters),
    )
    .unwrap()
    .credentials(smtp::authentication::Credentials::new(
        vars::smtp_username(),
        vars::smtp_password(),
    ))
    .connection_reuse(smtp::ConnectionReuseParameters::ReuseUnlimited)
    .transport();
    let result = mailer.send(email.into());
    if result.is_ok() {
        println!("Email sent");
        Ok(())
    } else {
        println!("Could not send email: {:?}", result);
        Err(AuthError::ProcessError(String::from(
            "Could not send confirmation email",
        )))
    }
}
