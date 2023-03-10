use serde::{Deserialize, Serialize};

/// stock_info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ExchStockInfo<'a> {
    #[serde(borrow)]
    #[serde(rename(deserialize = "pageHelp"))]
    pub page_help: ExchStockInfoPageHelp<'a>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ExchStockInfoPageHelp<'a> {
    #[serde(borrow)]
    pub data: Vec<ExchStockInfoData<'a>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ExchStockInfoData<'a> {
    #[serde(borrow)]
    #[serde(rename(deserialize = "COMPANY_ABBR"))]
    pub name: &'a str,

    #[serde(borrow)]
    #[serde(rename(deserialize = "A_STOCK_CODE"))]
    pub code: &'a str,

    #[serde(borrow)]
    #[serde(rename(deserialize = "DELIST_DATE"))]
    pub de_list: &'a str,

    #[serde(borrow)]
    #[serde(rename(deserialize = "LIST_DATE"))]
    pub list_date: &'a str,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct EastStockInfoMargin<'a> {
    #[serde(borrow)]
    pub data: EastStockInfoMarginData<'a>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct EastStockInfoMarginData<'a> {
    pub total: usize,
    #[serde(borrow)]
    pub diff: Vec<EastStockInfoMarginDataDetail<'a>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct EastStockInfoMarginDataDetail<'a> {
    #[serde(borrow)]
    #[serde(rename(deserialize = "f12"))]
    pub code: &'a str,
}

// stock index
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct EastStockIndex<'a> {
    #[serde(borrow)]
    pub data: Option<EastStockIndexData<'a>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct EastStockIndexData<'a> {
    pub total: usize,
    #[serde(borrow)]
    pub diff: Vec<EastStockIndexDataDetail<'a>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub(crate) enum EastStockIndexDataDetailValue<'a> {
    Float(f64),
    String(&'a str),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct EastStockIndexDataDetail<'a> {
    #[serde(borrow)]
    #[serde(rename(deserialize = "f12"))]
    pub code: &'a str,

    #[serde(borrow)]
    #[serde(rename(deserialize = "f14"))]
    pub name: &'a str,

    #[serde(borrow)]
    #[serde(rename(deserialize = "f2"))]
    pub price: EastStockIndexDataDetailValue<'a>,

    #[serde(borrow)]
    #[serde(rename(deserialize = "f9"))]
    pub pe: EastStockIndexDataDetailValue<'a>,

    #[serde(borrow)]
    #[serde(rename(deserialize = "f23"))]
    pub pb: EastStockIndexDataDetailValue<'a>,

    #[serde(borrow)]
    #[serde(rename(deserialize = "f20"))]
    pub total_value: EastStockIndexDataDetailValue<'a>,

    #[serde(borrow)]
    #[serde(rename(deserialize = "f21"))]
    pub currency_value: EastStockIndexDataDetailValue<'a>,
}

// stock index
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct EastStockIndustry<'a> {
    #[serde(borrow)]
    pub data: EastStockIndustryData<'a>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct EastStockIndustryData<'a> {
    pub total: usize,
    #[serde(borrow)]
    pub diff: Vec<EastStockIndustryDataDetail<'a>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct EastStockIndustryDataDetail<'a> {
    #[serde(borrow)]
    #[serde(rename(deserialize = "f12"))]
    pub code: &'a str,

    #[serde(borrow)]
    #[serde(rename(deserialize = "f14"))]
    pub name: &'a str,
}

/// stock_yjbb
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct EastStockYJBB<'a> {
    #[serde(borrow)]
    pub result: Option<EastStockYJBBResult<'a>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct EastStockYJBBResult<'a> {
    pub pages: usize,
    #[serde(borrow)]
    pub data: Vec<EastStockYJBBData<'a>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct EastStockYJBBData<'a> {
    #[serde(borrow)]
    #[serde(rename(deserialize = "SECURITY_CODE"))]
    pub code: &'a str,

    #[serde(borrow)]
    #[serde(rename(deserialize = "SECURITY_NAME_ABBR"))]
    pub name: &'a str,

    #[serde(borrow)]
    #[serde(rename(deserialize = "REPORTDATE"))]
    pub season_date: &'a str,

    #[serde(rename(deserialize = "BASIC_EPS"))]
    pub mg_sy: Option<f32>,

    #[serde(rename(deserialize = "TOTAL_OPERATE_INCOME"))]
    pub yysr: Option<f64>,

    #[serde(rename(deserialize = "YSTZ"))]
    pub yysr_tbzz: Option<f32>,

    #[serde(rename(deserialize = "YSHZ"))]
    pub yysr_jdhbzz: Option<f32>,

    #[serde(rename(deserialize = "PARENT_NETPROFIT"))]
    pub jlr: Option<f64>,

    #[serde(rename(deserialize = "SJLTZ"))]
    pub jlr_tbzz: Option<f32>,

    #[serde(rename(deserialize = "SJLHZ"))]
    pub jlr_jdhbzz: Option<f32>,

    #[serde(rename(deserialize = "BPS"))]
    pub mg_jzc: Option<f32>,

    #[serde(rename(deserialize = "WEIGHTAVG_ROE"))]
    pub jzc_syl: Option<f32>,

    #[serde(rename(deserialize = "MGJYXJJE"))]
    pub mg_jy_xjl: Option<f64>,

    #[serde(rename(deserialize = "XSMLL"))]
    pub xs_mll: Option<f32>,
}

/// stock_margin
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct EastStockMargin<'a> {
    #[serde(borrow)]
    pub result: Option<EastStockMarginResult<'a>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct EastStockMarginResult<'a> {
    pub pages: usize,
    #[serde(borrow)]
    pub data: Vec<EastStockMarginData<'a>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct EastStockMarginData<'a> {
    #[serde(borrow)]
    #[serde(rename(deserialize = "SCODE"))]
    pub code: &'a str,

    #[serde(borrow)]
    #[serde(rename(deserialize = "SECNAME"))]
    pub name: &'a str,

    #[serde(borrow)]
    #[serde(rename(deserialize = "DATE"))]
    pub trade_date: &'a str,

    /// 收盘价(元)(SPJ)
    #[serde(rename(deserialize = "SPJ"))]
    pub close: Option<f32>,
    /// 涨跌幅(%)(ZDF):
    #[serde(rename(deserialize = "ZDF"))]
    pub chg_pct: Option<f32>,
    /// 融资: 余额(元)(RZYE)
    #[serde(rename(deserialize = "RZYE"))]
    pub rz_ye: Option<f32>,
    /// 余额占流通市值比(%)(RZYEZB)
    #[serde(rename(deserialize = "RZYEZB"))]
    pub rz_ye_zb: Option<f32>,
    /// 买入额(元)(RZMRE)
    #[serde(rename(deserialize = "RZMRE"))]
    pub rz_mre: Option<f64>,
    ///	偿还额(元)(RZCHE)
    #[serde(rename(deserialize = "RZCHE"))]
    pub rz_che: Option<f64>,
    ///	净买入(元)(RZJME)
    #[serde(rename(deserialize = "RZJME"))]
    pub rz_jme: Option<f64>,
    /// 融券: 余额(元)(RQYE)
    #[serde(rename(deserialize = "RQYE"))]
    pub rq_ye: Option<f64>,
    ///	余量(股)(RQYL)
    #[serde(rename(deserialize = "RQYL"))]
    pub rq_yl: Option<i32>,
    /// 卖出量(股)(RQMCL)
    #[serde(rename(deserialize = "RQMCL"))]
    pub rq_mcl: Option<i32>,
    ///	偿还量(股)(RQCHL)
    #[serde(rename(deserialize = "RQCHL"))]
    pub rq_chl: Option<i32>,
    /// 净卖出(股)(RQJMG)
    #[serde(rename(deserialize = "RQJMG"))]
    pub rq_jmg: Option<i32>,
    /// 融资融券余额(元)(RZRQYE)
    #[serde(rename(deserialize = "RZRQYE"))]
    pub rz_rq_ye: Option<f64>,
    /// 融资融券余额差值(元)(RZRQYECZ)
    #[serde(rename(deserialize = "RZRQYECZ"))]
    pub rz_rq_ye_cz: Option<f64>,
}

/// stock_rt_quot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct XuQiuStockRtQuot<'a> {
    #[serde(borrow)]
    pub data: Option<Vec<XuQiuStockRtQuotData<'a>>>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct XuQiuStockRtQuotData<'a> {
    pub timestamp: i64,
    #[serde(borrow)]
    pub symbol: &'a str,
    pub last_close: f32,
    pub open: f32,
    pub high: f32,
    pub low: f32,
    #[serde(rename(deserialize = "current"))]
    pub last: f32,
    pub chg: f32,
    pub percent: f32,
    pub volume: i64,
    pub amount: f64,
    pub turnover_rate: f32,
    pub market_capital: f64,
    pub float_market_capital: f64,
    pub is_trade: bool,
}
