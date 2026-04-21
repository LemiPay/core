use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::HashMap;
use std::error::Error;

#[derive(Clone, Debug)]
struct DemoUser {
    name: String,
    email: String,
    password: String,
}

#[derive(Clone, Debug)]
struct Session {
    user: DemoUser,
    token: String,
    user_id: String,
    wallet_id: String,
    wallet_address: String,
    currency_id: String,
}

#[derive(Debug, Deserialize)]
struct LoginResponse {
    token: String,
}

#[derive(Debug, Deserialize)]
struct RegisterResponse {
    id: String,
}

#[derive(Debug, Deserialize)]
struct UserItem {
    id: String,
    email: String,
}

#[derive(Debug, Deserialize)]
struct GroupCreateResponse {
    id: String,
}

#[derive(Debug, Deserialize)]
struct WalletResponse {
    id: String,
    address: String,
    currency_id: String,
}

#[derive(Debug, Deserialize)]
struct GroupMemberResponse {
    user_id: String,
}

#[derive(Debug, Serialize)]
struct RegisterPayload<'a> {
    email: &'a str,
    password: &'a str,
    name: &'a str,
}

#[derive(Debug, Serialize)]
struct LoginPayload<'a> {
    email: &'a str,
    password: &'a str,
}

#[derive(Debug, Serialize)]
struct CreateGroupPayload<'a> {
    name: &'a str,
    description: &'a str,
}

#[derive(Debug, Serialize)]
struct CreateWalletPayload<'a> {
    address: &'a str,
    currency_ticker: &'a str,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let base_url =
        std::env::var("DEMO_API_BASE_URL").unwrap_or_else(|_| "http://127.0.0.1:3000".to_string());
    let client = Client::new();

    println!("== Demo populate by API ==");
    println!("Using API: {base_url}");

    let demo_users = vec![
        DemoUser {
            name: "Mateo".into(),
            email: "mateo@mateo".into(),
            password: "mateo".into(),
        },
        DemoUser {
            name: "Valentina Rossi".into(),
            email: "valentina.rossi@demo.app".into(),
            password: "demo1234".into(),
        },
        DemoUser {
            name: "Lautaro Benitez".into(),
            email: "lautaro.benitez@demo.app".into(),
            password: "demo1234".into(),
        },
        DemoUser {
            name: "Camila Ferrer".into(),
            email: "camila.ferrer@demo.app".into(),
            password: "demo1234".into(),
        },
        DemoUser {
            name: "Santiago Mendez".into(),
            email: "santiago.mendez@demo.app".into(),
            password: "demo1234".into(),
        },
        DemoUser {
            name: "Florencia Acosta".into(),
            email: "florencia.acosta@demo.app".into(),
            password: "demo1234".into(),
        },
    ];

    let mut sessions = Vec::new();
    for (idx, user) in demo_users.iter().enumerate() {
        register_or_login(&client, &base_url, user).await?;
        let token = login(&client, &base_url, user).await?;
        let user_id = fetch_user_id(&client, &base_url, &token, &user.email).await?;

        let wallet_address = format!(
            "0xDEMO{}{:02}",
            user.name.replace(' ', "").to_uppercase(),
            idx
        );
        let wallet = create_wallet_if_missing(&client, &base_url, &token, &wallet_address).await?;
        faucet_fund(&client, &base_url, &token, &wallet.id, "5000").await?;

        sessions.push(Session {
            user: user.clone(),
            token,
            user_id,
            wallet_id: wallet.id,
            wallet_address: wallet.address,
            currency_id: wallet.currency_id,
        });
    }

    let mateo = sessions
        .iter()
        .find(|s| s.user.email == "mateo@mateo")
        .ok_or("Mateo session not found")?
        .clone();
    let by_email: HashMap<String, Session> = sessions
        .iter()
        .cloned()
        .map(|s| (s.user.email.clone(), s))
        .collect();

    println!("\n== Creating groups ==");
    let travel_group_id = create_group(
        &client,
        &base_url,
        &mateo.token,
        "Viaje Bariloche 2026",
        "Plan del viaje y gastos",
    )
    .await?;
    let startup_group_id = create_group(
        &client,
        &base_url,
        &mateo.token,
        "Startup Casa Inteligente",
        "Gastos y compras del MVP",
    )
    .await?;
    let neighborhood_group_id = create_group(
        &client,
        &base_url,
        &by_email["santiago.mendez@demo.app"].token,
        "Vecinos Torre Norte",
        "Caja comun de edificio",
    )
    .await?;

    println!("\n== Sending and approving new-member proposals ==");
    invite_and_approve(
        &client,
        &base_url,
        &mateo,
        &by_email["valentina.rossi@demo.app"],
        &travel_group_id,
    )
    .await?;
    invite_and_approve(
        &client,
        &base_url,
        &mateo,
        &by_email["lautaro.benitez@demo.app"],
        &travel_group_id,
    )
    .await?;
    invite_and_approve(
        &client,
        &base_url,
        &mateo,
        &by_email["camila.ferrer@demo.app"],
        &travel_group_id,
    )
    .await?;

    invite_and_approve(
        &client,
        &base_url,
        &mateo,
        &by_email["santiago.mendez@demo.app"],
        &startup_group_id,
    )
    .await?;
    invite_and_approve(
        &client,
        &base_url,
        &mateo,
        &by_email["florencia.acosta@demo.app"],
        &startup_group_id,
    )
    .await?;
    invite_and_approve(
        &client,
        &base_url,
        &mateo,
        &by_email["camila.ferrer@demo.app"],
        &startup_group_id,
    )
    .await?;
    invite_and_approve(
        &client,
        &base_url,
        &by_email["santiago.mendez@demo.app"],
        &mateo,
        &neighborhood_group_id,
    )
    .await?;

    println!("\n== Creating group wallets ==");
    create_group_wallet_if_missing(
        &client,
        &base_url,
        &mateo.token,
        &travel_group_id,
        "0xGROUPTRAVELUSDC",
    )
    .await?;
    create_group_wallet_if_missing(
        &client,
        &base_url,
        &mateo.token,
        &startup_group_id,
        "0xGROUPSTARTUPUSDC",
    )
    .await?;
    create_group_wallet_if_missing(
        &client,
        &base_url,
        &by_email["santiago.mendez@demo.app"].token,
        &neighborhood_group_id,
        "0xGROUPNEIGHBORHOODUSDC",
    )
    .await?;

    println!("\n== Funding groups and creating activity ==");
    fund_group(
        &client,
        &base_url,
        &mateo.token,
        &travel_group_id,
        &mateo.wallet_address,
        &mateo.currency_id,
        "850",
        Some("Fondo inicial viaje: reserva cabaña"),
    )
    .await?;
    fund_group(
        &client,
        &base_url,
        &by_email["valentina.rossi@demo.app"].token,
        &travel_group_id,
        &by_email["valentina.rossi@demo.app"].wallet_address,
        &mateo.currency_id,
        "420",
        Some("Aporte para combustible y comida"),
    )
    .await?;

    create_expense(
        &client,
        &base_url,
        &by_email["camila.ferrer@demo.app"].token,
        &travel_group_id,
        &mateo.currency_id,
        "360",
        Some("Compra de supermercado para 4 dias"),
        vec![
            by_email["camila.ferrer@demo.app"].user_id.clone(),
            by_email["lautaro.benitez@demo.app"].user_id.clone(),
            by_email["valentina.rossi@demo.app"].user_id.clone(),
            mateo.user_id.clone(),
        ],
    )
    .await?;

    let travel_fund_round_id = create_fund_round(
        &client,
        &base_url,
        &mateo.token,
        &travel_group_id,
        &mateo.currency_id,
        "600",
    )
    .await?;
    contribute_fund_round(
        &client,
        &base_url,
        &mateo.token,
        &travel_fund_round_id,
        &mateo.wallet_id,
        "200",
    )
    .await?;
    contribute_fund_round(
        &client,
        &base_url,
        &by_email["valentina.rossi@demo.app"].token,
        &travel_fund_round_id,
        &by_email["valentina.rossi@demo.app"].wallet_id,
        "200",
    )
    .await?;
    contribute_fund_round(
        &client,
        &base_url,
        &by_email["lautaro.benitez@demo.app"].token,
        &travel_fund_round_id,
        &by_email["lautaro.benitez@demo.app"].wallet_id,
        "200",
    )
    .await?;

    fund_group(
        &client,
        &base_url,
        &mateo.token,
        &startup_group_id,
        &mateo.wallet_address,
        &mateo.currency_id,
        "1200",
        Some("Capital semilla para compras iniciales"),
    )
    .await?;
    create_expense(
        &client,
        &base_url,
        &by_email["santiago.mendez@demo.app"].token,
        &startup_group_id,
        &mateo.currency_id,
        "480",
        Some("Placas ESP32 + sensores"),
        vec![
            by_email["santiago.mendez@demo.app"].user_id.clone(),
            by_email["florencia.acosta@demo.app"].user_id.clone(),
            mateo.user_id.clone(),
        ],
    )
    .await?;

    println!("\n== Optional withdraw flow (proposal + execution) ==");
    let withdraw_proposal_id = create_withdraw_proposal(
        &client,
        &base_url,
        &mateo.token,
        &startup_group_id,
        &mateo.wallet_address,
        &mateo.currency_id,
        "150",
    )
    .await?;
    execute_withdraw(
        &client,
        &base_url,
        &mateo.token,
        &startup_group_id,
        &mateo.wallet_address,
        &mateo.currency_id,
        &withdraw_proposal_id,
    )
    .await?;

    println!("\nDemo dataset created successfully.");
    println!("Main login: mateo@mateo / mateo");
    Ok(())
}

async fn register_or_login(
    client: &Client,
    base_url: &str,
    user: &DemoUser,
) -> Result<(), Box<dyn Error>> {
    let register_url = format!("{base_url}/auth/register");
    let payload = RegisterPayload {
        email: &user.email,
        password: &user.password,
        name: &user.name,
    };

    let resp = client.post(register_url).json(&payload).send().await?;
    if resp.status().is_success() {
        let created: RegisterResponse = resp.json().await?;
        println!("Registered {} ({})", user.email, created.id);
    } else {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        println!("Register skipped for {} ({status}): {}", user.email, body);
    }
    Ok(())
}

async fn login(client: &Client, base_url: &str, user: &DemoUser) -> Result<String, Box<dyn Error>> {
    let login_url = format!("{base_url}/auth/login");
    let payload = LoginPayload {
        email: &user.email,
        password: &user.password,
    };

    let resp = client.post(login_url).json(&payload).send().await?;
    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(format!("Login failed for {} ({status}): {}", user.email, body).into());
    }
    let data: LoginResponse = resp.json().await?;
    Ok(data.token)
}

async fn fetch_user_id(
    client: &Client,
    base_url: &str,
    token: &str,
    email: &str,
) -> Result<String, Box<dyn Error>> {
    let users_url = format!("{base_url}/users");
    let resp = client.get(users_url).bearer_auth(token).send().await?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(format!("GET /users failed ({status}): {body}").into());
    }

    let users: Vec<UserItem> = resp.json().await?;
    users
        .into_iter()
        .find(|u| u.email == email)
        .map(|u| u.id)
        .ok_or_else(|| format!("User id not found for {email}").into())
}

async fn create_wallet_if_missing(
    client: &Client,
    base_url: &str,
    token: &str,
    address: &str,
) -> Result<WalletResponse, Box<dyn Error>> {
    let url = format!("{base_url}/wallet/create");
    let payload = CreateWalletPayload {
        address,
        currency_ticker: "USDC",
    };

    let resp = client
        .post(url)
        .bearer_auth(token)
        .json(&payload)
        .send()
        .await?;

    if resp.status().is_success() {
        return Ok(resp.json().await?);
    }

    let status = resp.status();
    let body = resp.text().await.unwrap_or_default();
    println!("Wallet create skipped ({status}): {body}");

    let lookup_url = format!("{base_url}/wallet/{address}?currency=USDC");
    let lookup = client.get(lookup_url).bearer_auth(token).send().await?;
    if lookup.status().is_success() {
        return Ok(lookup.json().await?);
    }

    // If the deterministic address doesn't exist, reuse any existing USDC wallet.
    let all_wallets_url = format!("{base_url}/wallet/get-all");
    let all_wallets_resp = client
        .get(all_wallets_url)
        .bearer_auth(token)
        .send()
        .await?;
    if !all_wallets_resp.status().is_success() {
        let lookup_status = lookup.status();
        let lookup_body = lookup.text().await.unwrap_or_default();
        let list_status = all_wallets_resp.status();
        let list_body = all_wallets_resp.text().await.unwrap_or_default();
        return Err(format!(
            "Wallet lookup/list failed ({lookup_status}/{list_status}): {lookup_body} / {list_body}"
        )
        .into());
    }

    let grouped_wallets: Vec<Value> = all_wallets_resp.json().await?;
    for group in grouped_wallets {
        if let Some(currencies) = group.get("currencies").and_then(|v| v.as_array()) {
            for wallet in currencies {
                let ticker = wallet
                    .get("ticker")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default();
                if ticker != "USDC" {
                    continue;
                }

                let wallet_id = wallet
                    .get("wallet_id")
                    .and_then(|v| v.as_str())
                    .ok_or("wallet_id missing in /wallet/get-all response")?;
                let wallet_address = wallet
                    .get("address")
                    .and_then(|v| v.as_str())
                    .ok_or("address missing in /wallet/get-all response")?;
                let currency_id = wallet
                    .get("currency_id")
                    .and_then(|v| v.as_str())
                    .ok_or("currency_id missing in /wallet/get-all response")?;

                return Ok(WalletResponse {
                    id: wallet_id.to_string(),
                    address: wallet_address.to_string(),
                    currency_id: currency_id.to_string(),
                });
            }
        }
    }

    Err("No USDC wallet found for user after create fallback".into())
}

async fn faucet_fund(
    client: &Client,
    base_url: &str,
    token: &str,
    wallet_id: &str,
    amount: &str,
) -> Result<(), Box<dyn Error>> {
    let url = format!("{base_url}/wallet/fund/{wallet_id}");
    let resp = client
        .post(url)
        .bearer_auth(token)
        .json(&json!({ "amount": amount }))
        .send()
        .await?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(format!("Faucet fund failed ({status}): {body}").into());
    }
    Ok(())
}

async fn create_group(
    client: &Client,
    base_url: &str,
    token: &str,
    name: &str,
    description: &str,
) -> Result<String, Box<dyn Error>> {
    let url = format!("{base_url}/group/create");
    let payload = CreateGroupPayload { name, description };
    let resp = client
        .post(url)
        .bearer_auth(token)
        .json(&payload)
        .send()
        .await?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(format!("Create group failed ({status}): {body}").into());
    }
    let group: GroupCreateResponse = resp.json().await?;
    println!("Created group '{name}' => {}", group.id);
    Ok(group.id)
}

async fn invite_and_approve(
    client: &Client,
    base_url: &str,
    inviter: &Session,
    invited: &Session,
    group_id: &str,
) -> Result<(), Box<dyn Error>> {
    let create_url = format!("{base_url}/proposal/new-member/{group_id}");
    let create_resp = client
        .post(create_url)
        .bearer_auth(&inviter.token)
        .json(&json!({ "user_email": invited.user.email }))
        .send()
        .await?;

    if !create_resp.status().is_success() {
        let status = create_resp.status();
        let body = create_resp.text().await.unwrap_or_default();
        if is_user_in_group(client, base_url, &inviter.token, group_id, &invited.user_id).await? {
            println!(
                "User {} already in group {}, skipping invite",
                invited.user.email, group_id
            );
            return Ok(());
        }
        return Err(format!("Create invitation failed ({status}): {body}").into());
    }

    let invited_received = client
        .get(format!("{base_url}/proposal/received"))
        .bearer_auth(&invited.token)
        .send()
        .await?;

    if !invited_received.status().is_success() {
        let status = invited_received.status();
        let body = invited_received.text().await.unwrap_or_default();
        return Err(format!("Read received proposals failed ({status}): {body}").into());
    }

    let proposals: Vec<Value> = invited_received.json().await?;
    // The API response shape can evolve; match robustly by group and sender.
    let maybe_id = proposals.into_iter().find_map(|p| {
        let p_group_id = p
            .pointer("/proposal/group_id")
            .and_then(|v| v.as_str())
            .or_else(|| p.get("group_id").and_then(|v| v.as_str()));
        let p_created_by = p
            .pointer("/proposal/created_by")
            .and_then(|v| v.as_str())
            .or_else(|| p.get("created_by").and_then(|v| v.as_str()));
        let p_id = p
            .pointer("/proposal/id")
            .and_then(|v| v.as_str())
            .or_else(|| p.get("proposal_id").and_then(|v| v.as_str()))
            .or_else(|| p.get("id").and_then(|v| v.as_str()));

        match (p_group_id, p_created_by, p_id) {
            (Some(g), Some(created_by), Some(id))
                if g == group_id && created_by == inviter.user_id =>
            {
                Some(id.to_string())
            }
            _ => None,
        }
    });

    let proposal_id = match maybe_id {
        Some(id) => id,
        None => {
            if is_user_in_group(client, base_url, &inviter.token, group_id, &invited.user_id)
                .await?
            {
                println!(
                    "User {} already in group {} after invite",
                    invited.user.email, group_id
                );
                return Ok(());
            }
            return Err(format!(
                "Proposal not found for {} in group {}",
                invited.user.email, group_id
            )
            .into());
        }
    };

    let respond_url = format!("{base_url}/proposal/respond_proposal/{proposal_id}");
    let respond_resp = client
        .put(respond_url)
        .bearer_auth(&invited.token)
        .json(&json!({ "response": true }))
        .send()
        .await?;

    if !respond_resp.status().is_success() {
        let status = respond_resp.status();
        let body = respond_resp.text().await.unwrap_or_default();
        return Err(format!("Approve invitation failed ({status}): {body}").into());
    }

    println!(
        "Approved invitation: {} -> group {}",
        invited.user.email, group_id
    );
    if !is_user_in_group(client, base_url, &inviter.token, group_id, &invited.user_id).await? {
        return Err(format!(
            "User {} still not in group {} after approval",
            invited.user.email, group_id
        )
        .into());
    }
    Ok(())
}

async fn is_user_in_group(
    client: &Client,
    base_url: &str,
    token: &str,
    group_id: &str,
    user_id: &str,
) -> Result<bool, Box<dyn Error>> {
    let url = format!("{base_url}/group/{group_id}/members");
    let resp = client.get(url).bearer_auth(token).send().await?;
    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(format!("Group members read failed ({status}): {body}").into());
    }
    let members: Vec<GroupMemberResponse> = resp.json().await?;
    Ok(members.into_iter().any(|m| m.user_id == user_id))
}

async fn create_group_wallet_if_missing(
    client: &Client,
    base_url: &str,
    token: &str,
    group_id: &str,
    address: &str,
) -> Result<(), Box<dyn Error>> {
    let create_url = format!("{base_url}/group-wallet/{group_id}/create");

    let first_resp = client
        .post(&create_url)
        .bearer_auth(token)
        .json(&json!({
            "address": address,
            "currency_ticker": "USDC"
        }))
        .send()
        .await?;

    if first_resp.status().is_success() {
        return Ok(());
    }

    // Retry once with an unique address derived from group_id to avoid collisions
    // across repeated demo runs (address+currency must be unique).
    let group_suffix: String = group_id.chars().filter(|c| *c != '-').take(12).collect();
    let retry_address = format!("{address}_{group_suffix}");

    let retry_resp = client
        .post(&create_url)
        .bearer_auth(token)
        .json(&json!({
            "address": retry_address,
            "currency_ticker": "USDC"
        }))
        .send()
        .await?;

    if retry_resp.status().is_success() {
        return Ok(());
    }

    let list_url = format!("{base_url}/group-wallet/{group_id}");
    let list = client.get(list_url).bearer_auth(token).send().await?;
    if !list.status().is_success() {
        let status = list.status();
        let body = list.text().await.unwrap_or_default();
        return Err(format!("Group wallet create/list failed ({status}): {body}").into());
    }

    let wallets: Vec<Value> = list.json().await?;
    // For idempotency, accept either the exact address or any existing group wallet.
    let exists = wallets.into_iter().any(|w| {
        let same_address = w
            .get("address")
            .and_then(|v| v.as_str())
            .map(|a| a == address)
            .unwrap_or(false);
        let has_usdc = w
            .get("currency_id")
            .and_then(|v| v.as_str())
            .map(|currency_id| !currency_id.is_empty())
            .unwrap_or(false);
        same_address || has_usdc
    });
    if !exists {
        return Err(format!("Group wallet was not created and not found for {group_id}").into());
    }
    Ok(())
}

async fn fund_group(
    client: &Client,
    base_url: &str,
    token: &str,
    group_id: &str,
    user_wallet_address: &str,
    currency_id: &str,
    amount: &str,
    description: Option<&str>,
) -> Result<(), Box<dyn Error>> {
    let url = format!("{base_url}/transaction/{group_id}/fund");
    let resp = client
        .post(url)
        .bearer_auth(token)
        .json(&json!({
            "amount": amount,
            "address": user_wallet_address,
            "currency_id": currency_id,
            "description": description
        }))
        .send()
        .await?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(format!("Fund group failed ({status}): {body}").into());
    }
    Ok(())
}

async fn create_expense(
    client: &Client,
    base_url: &str,
    token: &str,
    group_id: &str,
    currency_id: &str,
    amount: &str,
    description: Option<&str>,
    participants: Vec<String>,
) -> Result<(), Box<dyn Error>> {
    let payload_participants: Vec<Value> = participants
        .into_iter()
        .map(|user_id| json!({ "user_id": user_id }))
        .collect();
    let url = format!("{base_url}/expense/new/{group_id}");
    let resp = client
        .post(url)
        .bearer_auth(token)
        .json(&json!({
            "currency_id": currency_id,
            "amount": amount,
            "description": description,
            "participants": payload_participants
        }))
        .send()
        .await?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(format!("Create expense failed ({status}): {body}").into());
    }
    Ok(())
}

async fn create_fund_round(
    client: &Client,
    base_url: &str,
    token: &str,
    group_id: &str,
    currency_id: &str,
    target_amount: &str,
) -> Result<String, Box<dyn Error>> {
    let url = format!("{base_url}/group-wallet/fund-round/create/{group_id}");
    let resp = client
        .post(url)
        .bearer_auth(token)
        .json(&json!({
            "target_amount": target_amount,
            "currency_id": currency_id
        }))
        .send()
        .await?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(format!("Create fund round failed ({status}): {body}").into());
    }

    let body: Value = resp.json().await?;
    let proposal_id = body
        .pointer("/proposal/id")
        .and_then(|v| v.as_str())
        .ok_or("Missing proposal.id in fund round response")?;
    Ok(proposal_id.to_string())
}

async fn contribute_fund_round(
    client: &Client,
    base_url: &str,
    token: &str,
    fund_round_id: &str,
    sender_wallet_id: &str,
    amount: &str,
) -> Result<(), Box<dyn Error>> {
    let url = format!("{base_url}/group-wallet/fund-round/{fund_round_id}/contribute");
    let resp = client
        .post(url)
        .bearer_auth(token)
        .json(&json!({
            "amount": amount,
            "sender_wallet_id": sender_wallet_id
        }))
        .send()
        .await?;
    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(format!("Contribute fund round failed ({status}): {body}").into());
    }
    Ok(())
}

async fn create_withdraw_proposal(
    client: &Client,
    base_url: &str,
    token: &str,
    group_id: &str,
    address: &str,
    currency_id: &str,
    amount: &str,
) -> Result<String, Box<dyn Error>> {
    let url = format!("{base_url}/transaction/{group_id}/withdraw/proposal");
    let resp = client
        .post(url)
        .bearer_auth(token)
        .json(&json!({
            "address": address,
            "currency_id": currency_id,
            "amount": amount
        }))
        .send()
        .await?;
    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(format!("Create withdraw proposal failed ({status}): {body}").into());
    }

    let body: Value = resp.json().await?;
    let proposal_id = body
        .pointer("/proposal/id")
        .and_then(|v| v.as_str())
        .ok_or("Missing proposal.id in withdraw proposal response")?;
    Ok(proposal_id.to_string())
}

async fn execute_withdraw(
    client: &Client,
    base_url: &str,
    token: &str,
    group_id: &str,
    address: &str,
    currency_id: &str,
    proposal_id: &str,
) -> Result<(), Box<dyn Error>> {
    let url = format!("{base_url}/transaction/{group_id}/withdraw/execute");
    let resp = client
        .post(url)
        .bearer_auth(token)
        .json(&json!({
            "address": address,
            "proposal_id": proposal_id,
            "currency_id": currency_id
        }))
        .send()
        .await?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(format!("Execute withdraw failed ({status}): {body}").into());
    }
    Ok(())
}
