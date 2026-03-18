struct TradingPair(String);

impl TradingPair {
    fn new(pair: impl Into<String>) -> Result<Self, String> {
        let pair = pair.into();
        if pair.contains('/') && pair.split('/').count() == 2 {
            Ok(Self(pair))
        } else {
            Err("Invalid trading pair format. Expected 'BASE/QUOTE'".to_string())
        }
    }

    fn base(&self) -> &str {
        self.0.split('/').next().unwrap()
    }

    fn quote(&self) -> &str {
        self.0.split('/').nth(1).unwrap()
    }

    fn as_str(&self) -> &str {
        &self.0
    }
}

struct TradingStrategyConfig {
    pair: TradingPair,
    initial_balance: u64,
    max_position_size: u64,
    stop_loss_percent: Option<f64>,
    take_profit_percent: Option<f64>,
    enable_short: bool,
}

struct TradingStrategyConfigBuilder {
    pair: Option<TradingPair>,
    initial_balance: Option<u64>,
    max_position_size: Option<u64>,
    stop_loss_percent: Option<f64>,
    take_profit_percent: Option<f64>,
    enable_short: bool,
}

impl TradingStrategyConfigBuilder {
    fn new() -> Self {
        Self {
            pair: None,
            initial_balance: None,
            max_position_size: None,
            stop_loss_percent: None,
            take_profit_percent: None,
            enable_short: false,
        }
    }

    fn pair(mut self, pair: TradingPair) -> Self {
        self.pair = Some(pair);
        self
    }

    fn initial_balance(mut self, balance: u64) -> Self {
        self.initial_balance = Some(balance);
        self
    }

    fn max_position_size(mut self, size: u64) -> Self {
        self.max_position_size = Some(size);
        self
    }

    fn stop_loss(mut self, percent: f64) -> Self {
        self.stop_loss_percent = Some(percent);
        self
    }

    fn take_profit(mut self, percent: f64) -> Self {
        self.take_profit_percent = Some(percent);
        self
    }

    fn enable_short(mut self) -> Self {
        self.enable_short = true;
        self
    }

    fn build(self) -> Result<TradingStrategyConfig, String> {
        Ok(TradingStrategyConfig {
            pair: self.pair.ok_or("pair is required")?,
            initial_balance: self.initial_balance.ok_or("initial_balance is required")?,
            max_position_size: self
                .max_position_size
                .ok_or("max_position_size is required")?,
            stop_loss_percent: self.stop_loss_percent,
            take_profit_percent: self.take_profit_percent,
            enable_short: self.enable_short,
        })
    }
}

struct Init;
struct Ready {
    config: TradingStrategyConfig,
}
struct Running {
    config: TradingStrategyConfig,
    current_balance: u64,
    positions: Vec<Position>,
}
struct Stopped {
    reason: String,
    final_balance: u64,
}

struct Position {
    pair: TradingPair,
    size: u64,
    entry_price: f64,
}

impl Init {
    fn new() -> Self {
        Self
    }

    fn configure(self, config: TradingStrategyConfig) -> Ready {
        Ready { config }
    }
}

impl Ready {
    fn config(&self) -> &TradingStrategyConfig {
        &self.config
    }

    fn start(self) -> Running {
        Running {
            current_balance: self.config.initial_balance,
            config: self.config,
            positions: Vec::new(),
        }
    }
}

impl Running {
    fn stop(self, reason: String) -> Stopped {
        Stopped {
            reason,
            final_balance: self.current_balance,
        }
    }

    fn execute_trade(&mut self, pair: TradingPair, size: u64, price: f64) {
        self.positions.push(Position {
            pair,
            size,
            entry_price: price,
        });
    }

    fn current_balance(&self) -> u64 {
        self.current_balance
    }
}

impl Stopped {
    fn reason(&self) -> &str {
        &self.reason
    }

    fn final_balance(&self) -> u64 {
        self.final_balance
    }

    fn restart(self, config: TradingStrategyConfig) -> Ready {
        Ready { config }
    }
}

fn main() -> Result<(), String> {
    let pair = TradingPair::new("BTC/USD")?;
    println!(
        "Trading pair: {} (base: {}, quote: {})",
        pair.as_str(),
        pair.base(),
        pair.quote()
    );

    let config = TradingStrategyConfigBuilder::new()
        .pair(pair)
        .initial_balance(10000)
        .max_position_size(1000)
        .stop_loss(5.0)
        .take_profit(10.0)
        .enable_short()
        .build()?;

    let bot = Init::new();
    let ready = bot.configure(config);

    let mut running = ready.start();

    let trade_pair = TradingPair::new("ETH/USD")?;
    running.execute_trade(trade_pair, 100, 2000.0);

    let stopped = running.stop("Manual stop".to_string());
    println!(
        "Stopped: {}, Final balance: {}",
        stopped.reason(),
        stopped.final_balance()
    );

    Ok(())
}
