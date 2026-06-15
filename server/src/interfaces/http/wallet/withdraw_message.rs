use alloy::primitives::Address;
use uuid::Uuid;

pub fn build_withdraw_authorization_message(
    wallet_id: Uuid,
    amount: &str,
    address: &Address,
    uri: &str,
) -> String {
    format!(
        "lemipay.app quiere autorizar un retiro:\n\nWallet: {}\nMonto: {}\nAddress: {}\n\nURI: {}",
        wallet_id, amount, address, uri
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn message_format_matches_client_template() {
        let wallet_id = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        let address: Address = "0xAb5801a7D398351b8bE11C439e08FE5E325a6E2a"
            .parse()
            .unwrap();
        let message = build_withdraw_authorization_message(
            wallet_id,
            "10",
            &address,
            "http://localhost:5173",
        );

        assert_eq!(
            message,
            "lemipay.app quiere autorizar un retiro:\n\nWallet: 550e8400-e29b-41d4-a716-446655440000\nMonto: 10\nAddress: 0xab5801a7D398351B8be11C439E08fe5e325A6E2a\n\nURI: http://localhost:5173"
        );
    }
}
