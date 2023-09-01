use bridge_runtime_common::messages_xcm_extension::XcmBlobHauler;
use datagen_runtime::{
	BridgeRialtoMessagesConfig,
	BridgeRialtoParachainMessagesConfig, BridgeWestendGrandpaConfig,
	AccountId, AuraConfig,BeefyConfig, BalancesConfig, GenesisConfig, GrandpaConfig, Signature, SudoConfig,
	SystemConfig, SessionConfig, WASM_BINARY, NodeAuthorizationConfig
};
use sc_service::ChainType;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{sr25519, Pair, Public, OpaquePeerId};
use sc_consensus_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{IdentifyAccount, Verify};
use datagen_runtime::opaque::SessionKeys;

/// "Name" of the account, which owns the with-Westend GRANDPA pallet.
const WESTEND_GRANDPA_PALLET_OWNER: &str = "Westend.GrandpaOwner";
/// "Name" of the account, which owns the with-Rialto messages pallet.
const RIALTO_MESSAGES_PALLET_OWNER: &str = "Rialto.MessagesOwner";
/// "Name" of the account, which owns the with-RialtoParachain messages pallet.
const RIALTO_PARACHAIN_MESSAGES_PALLET_OWNER: &str = "RialtoParachain.MessagesOwner";

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate an Aura authority key.
pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
	(get_from_seed::<AuraId>(s), get_from_seed::<GrandpaId>(s))
}

pub fn development_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || {
			testnet_genesis(
				wasm_binary,
				vec![
					(
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						get_from_seed::<AuraId>("Alice"),
						get_from_seed::<GrandpaId>("Alice"),
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Bob"),
						get_from_seed::<AuraId>("Bob"),
						get_from_seed::<GrandpaId>("Bob"),
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Charlie"),
						get_from_seed::<AuraId>("Charlie"),
						get_from_seed::<GrandpaId>("Charlie"),
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Dave"),
						get_from_seed::<AuraId>("Dave"),
						get_from_seed::<GrandpaId>("Dave"),
					),
				],
				// Initial PoA authorities
				vec![authority_keys_from_seed("Alice")],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
				],
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		None,
		// Properties
		None,
		// Extensions
		None,
	))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;



	Ok(ChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"local_testnet",
		ChainType::Local,
		move || {
			testnet_genesis(
				wasm_binary,
					vec![
					(
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						get_from_seed::<AuraId>("Alice"),
						get_from_seed::<GrandpaId>("Alice"),
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Bob"),
						get_from_seed::<AuraId>("Bob"),
						get_from_seed::<GrandpaId>("Bob"),
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Charlie"),
						get_from_seed::<AuraId>("Charlie"),
						get_from_seed::<GrandpaId>("Charlie"),
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Dave"),
						get_from_seed::<AuraId>("Dave"),
						get_from_seed::<GrandpaId>("Dave"),
					),
				],
				// Initial PoA authorities
				vec![
					authority_keys_from_seed("Alice"),
					authority_keys_from_seed("Bob"),
					authority_keys_from_seed("Charlie"),
					authority_keys_from_seed("Dave")
					],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
					get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
					get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
				],
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		None,
		None,
		// Extensions
		None,
	))
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	wasm_binary: &[u8],
	authorities: Vec<(AccountId, AuraId, GrandpaId)>,
	initial_authorities: Vec<(AuraId, GrandpaId)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	_enable_println: bool,
) -> GenesisConfig {
	GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
			..Default::default()
		},
		session: SessionConfig {
			keys: authorities
				.iter()
				.map(|x| (x.0.clone(), x.0.clone(), session_keys(x.1.clone(), x.2.clone())))
				.collect::<Vec<_>>(),
		},
		balances: BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts.iter().cloned().map(|k| (k, 1 << 60)).collect(),
		},
		beefy: BeefyConfig::default(),
		aura: AuraConfig {
			authorities: vec![]
		},
		node_authorization: NodeAuthorizationConfig {
			nodes: vec![
				(
					OpaquePeerId(bs58::decode("12D3KooWBmAwcd4PJNJvfV89HwE48nwkRmAgo8Vy3uQEyNNHBox2").into_vec().unwrap()),
					endowed_accounts[0].clone(),
				),
				(
					OpaquePeerId(bs58::decode("12D3KooWQYV9dGMFoRzNStwpXztXaBUjtPqi6aU76ZgUriHhKust").into_vec().unwrap()),
					endowed_accounts[1].clone(),
				),
				(
					OpaquePeerId(bs58::decode("12D3KooWJvyP3VJYymTqG7eH4PM5rN4T2agk5cdNCfNymAqwqcvZ").into_vec().unwrap()),
					endowed_accounts[2].clone(),
				),
				(
					OpaquePeerId(bs58::decode("12D3KooWPHWFrfaJzxPnqnAYAoRUyAHHKqACmEycGTVmeVhQYuZN").into_vec().unwrap()),
					endowed_accounts[3].clone(),
				)
			]
		},
		grandpa: GrandpaConfig {
			authorities: initial_authorities.iter().map(|x| (x.1.clone(), 1)).collect(),
			..Default::default()
		},
		sudo: SudoConfig {
			// Assign network admin rights.
			key: Some(root_key),
		},
		transaction_payment: Default::default(),
		bridge_westend_grandpa: BridgeWestendGrandpaConfig {
			// for our deployments to avoid multiple same-nonces transactions:
			// //Alice is already used to initialize Rialto<->Millau bridge
			// => let's use //Westend.GrandpaOwner to initialize Westend->Millau bridge
			owner: Some(get_account_id_from_seed::<sr25519::Public>(WESTEND_GRANDPA_PALLET_OWNER)),
			..Default::default()
		},
		bridge_rialto_messages: BridgeRialtoMessagesConfig {
			owner: Some(get_account_id_from_seed::<sr25519::Public>(RIALTO_MESSAGES_PALLET_OWNER)),
			opened_lanes: vec![datagen_runtime::rialto_messages::ToRialtoXcmBlobHauler::xcm_lane()],
			..Default::default()
		},
		bridge_rialto_parachain_messages: BridgeRialtoParachainMessagesConfig {
			owner: Some(get_account_id_from_seed::<sr25519::Public>(
				RIALTO_PARACHAIN_MESSAGES_PALLET_OWNER,
			)),
			opened_lanes: vec![
				datagen_runtime::datagen_parachain_messages::ToDatagenParachainXcmBlobHauler::xcm_lane(
				),
			],
			..Default::default()
		},
		xcm_pallet: Default::default(),
	}
}

/// Helper for session keys to map aura id
fn session_keys(aura: AuraId, grandpa: GrandpaId) -> SessionKeys {
	SessionKeys { aura, grandpa }
}
