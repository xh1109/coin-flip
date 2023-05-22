use near_sdk::{
    borsh, collections::UnorderedMap, env, json_types::U128, near_bindgen, AccountId, Balance,
};

/// 定义内存分配机制
///
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
/// 1 NEAR 单位
const ONE_NEAR: u128 = 1_000_000_000_000_000_000_000_000;
// 比对常量
const PROB: u8 = 128;

/// 老虎机
// #[near_bindgen] 用来表示这是 near 合约
// 每个合约都必须有 near_bindgen 这个宏
#[near_bindgen]
// BorshDeserialize，BorshSerialize 为 SlotMachine 实现二进制的序列化和反序列化
#[derive(borsh::BorshDeserialize, borsh::BorshSerialize)]
pub struct SlotMachine {
    pub owner_id: AccountId,
    pub credits: UnorderedMap<AccountId, Balance>,
}

impl Default for SlotMachine {
    /// 不提供默认实现
    fn default() -> Self {
        panic!("Should be initialized before usage")
    }
}

#[near_bindgen]
impl SlotMachine {
    /// 合约初始化
    // init 宏命令来进行合约初始化，默认情况下如果合约尚未初始化，所有的合约方法都将尝试初始化合约状态
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        assert!(
            env::is_valid_account_id(owner_id.as_bytes()),
            "Invalid owner account"
        );
        assert!(!env::state_exists(), "Already initialized");
        Self {
            owner_id,
            credits: UnorderedMap::new(b"credits".to_vec()),
        }
    }

    /// 定金
    // 如果合约方法没有使用 payable/near_bindgen 宏命令进行修饰。那么就无法使用合约上金钱相关的功能。
    #[payable]
    pub fn deposit(&mut self) {
        // 获取账户id ，签署交易的账户/发布合约的账户
        let account_id = env::signer_account_id();
        // 获取余额
        let deposit = env::attached_deposit();
        // 获取当前账户的积分
        let mut credits = self.credits.get(&account_id).unwrap_or(0);
        credits = credits + deposit;
        // 更新积分
        self.credits.insert(&account_id, &credits);
    }

    pub fn play(&mut self) -> u8 {
        let account_id = env::signer_account_id();
        let mut credits = self.credits.get(&account_id).unwrap_or(0);
        // 如果积分小于等于0 则无法进行
        assert!(credits > 0, "no credits to play");
        // 每次消耗 1 NEAR
        credits = credits - ONE_NEAR;

        // 获取一个随机数据
        let rand: u8 = *env::random_seed().get(0).unwrap();
        // 如果随机出来的数据小于用于比对的 PROB 常量，则执行增加积分的操作
        if rand < PROB {
            credits = credits + 10 * ONE_NEAR;
        }
        // 更新积分
        self.credits.insert(&account_id, &credits);
        rand
    }

    /// 外部获取积分的方法
    pub fn get_credits(&self, account_id: AccountId) -> U128 {
        self.credits.get(&account_id).unwrap_or(0).into()
    }
}
