use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint256, Binary};
//     struct DeploymentConfig {
//         string name;
//         string symbol;
//         uint256 maxSupply;
//         address payable treasuryAddress;
//     }

//     struct RuntimeConfig {
//         string baseTokenURI;
//         string baseTokenURIExtension;
//         uint256 mintPrice;
//         uint256 saleStartTime;
//         uint256 saleEndTime;
//         string prerevealTokenURI;
//         uint256 protocolFee;
//     }
#[cw_serde]
pub struct InstantiateMsg {
    pub name: String,
    pub symbol: String,
    pub description: Option<String>,
    pub uri: Option<String>,
    pub uri_hash: Option<String>,
    pub data: Option<Binary>,
    pub features: Option<Vec<u32>>,
    pub royalty_rate: Option<String>,
    pub treasury_address: Addr,
    pub protocol_address: Addr,
    pub current_token_id: Uint256,
    pub mint_price: Uint256,
    pub sale_start_time: Uint256,
    pub sale_end_time: Uint256,
    pub protocol_fee: Uint256,
    pub max_total_mint: Uint256,
    pub prereveal_token_uri: String,
    pub uri_status: bool,
}

#[cw_serde]
pub enum ExecuteMsg {
    Purchase{
        count: Uint256,
        id: String,
        uri: Option<String>,
        uri_hash: Option<String>,
        data: Option<Binary>,
        receiver: String,
    },
    MintAndSend { account: String, amount: u128 },
    Mint {
        id: String,
        uri: Option<String>,
        uri_hash: Option<String>,
        data: Option<Binary>,
    },
    Burn {
        id: String,
    },
    Freeze {
        id: String,
    },
    Unfreeze {
        id: String,
    },
    AddToWhitelist {
        id: String,
        account: String,
    },
    RemoveFromWhitelist {
        id: String,
        account: String,
    },
    Send {
        id: String,
        receiver: String,
    },
}

#[cw_serde]
pub enum QueryMsg {
    Params {},
    Class {},
    Classes { issuer: String },
    Frozen { id: String },
    Whitelisted { id: String, account: String },
    WhitelistedAccountsForNft { id: String },
    Balance { owner: String },
    Owner { id: String },
    Supply {},
    Nft { id: String }, // we use Nft not NFT since NFT is decoded as n_f_t
    Nfts { owner: Option<String> }, // we use Nfts not NFTs since NFTs is decoded as n_f_ts
    ClassNft {}, // we use ClassNft instead of Class because there is already a Class query being used
    ClassesNft {}, // we use ClassesNft instead of Class because there is already a Classes query being used
    BurntNft { nft_id: String },
    BurntNftsInClass {},
    GetInfo {owner: String},
}
