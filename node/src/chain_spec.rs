use node_template_runtime::{
	AccountId, AuraConfig, BalancesConfig, GrandpaConfig, RuntimeGenesisConfig, Signature,
	SudoConfig, SystemConfig, WASM_BINARY,
};
use sc_service::ChainType;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_core::{sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<RuntimeGenesisConfig>;

/// Generate a crypto pair from seed.
/// 看上去似乎是从字符串形式的种子中生成了一堆密钥，并返回了公钥。
/// 很显然，计算公钥的方法和这个TPublic泛型参数密切相关。
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic/* TPublic是泛型参数，Public是trait */::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
/// 先通过种子获得公钥，再凭借公钥找到账号，最后返回账号ID。
/// 因此记得，先有种子，再有不同共识算法下的公钥，最后才有AccoundId
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate an Aura authority key.
/// 调用get_from_seed两次（一次在AuraId上计算公钥，一次在GrandpaId上计算公钥），
/// 返回计算获得的两个公钥。
pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
	(get_from_seed::<AuraId>(s), get_from_seed::<GrandpaId>(s))
}

// 以下几个函数比较相似，都是创建了一些不同用途的链的配置文件
// 这些配置文件规定了链的名字，Id等，还规定了链中包含的账号名字、超级用户的名字等
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
				// Initial PoA authorities
				vec![authority_keys_from_seed("Alice"), authority_keys_from_seed("Bob")],
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
/// 这个函数是为上面两个函数服务的、
fn testnet_genesis(
	wasm_binary: &[u8], // 来源于WASM_BINARY.ok_or_else(...)
	initial_authorities: Vec<(AuraId, GrandpaId)>, // 一个列表，其中包含PoA共识算法所需要的验证者（authority）的Id对，由同一个种子在AuraId和GrandpaId中计算获得的两个公钥构成。
	root_key: AccountId, // 根用户，管理员，通常是通过调用get_account_id_from_seed从给定seed中算出来的
	endowed_accounts: Vec<AccountId>, // 所有预定用户的列表
	_enable_println: bool,
) -> RuntimeGenesisConfig {
	RuntimeGenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
			..Default::default() // 剩下的全用默认情况
		},
		balances: BalancesConfig {
			// 给所有预定用户的钱包疯狂氪金
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts.iter().cloned().map(|k| (k, 1 << 60)).collect(),
		},
		aura: AuraConfig {
			// 取PoA的所有验证者，然后收集他们在AuraId上的公钥
			authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
		},
		grandpa: GrandpaConfig {
			// 取PoA的所有验证者，然后收集他们在GrandpaId上的公钥
			authorities: initial_authorities.iter().map(|x| (x.1.clone(), 1/*? 这个1是做什么的呢？ */)).collect(),
			..Default::default()
		},
		sudo: SudoConfig {
			// 设置超级管理员
			key: Some(root_key),
		},
		transaction_payment: Default::default(),
	}
}
