use verilated_module::module;

#[module(top)]
pub struct Top {
  #[port(clock)]
  pub clock: bool,
  #[port(reset)]
  pub reset: bool,
  // axi aw
  #[port(output)]
  pub MAXI_awid: [bool; 3],
  #[port(output)]
  pub MAXI_awaddr: [bool; 32],
  #[port(output)]
  pub MAXI_awlen: [bool; 8],
  #[port(output)]
  pub MAXI_awsize: [bool; 3],
  #[port(output)]
  pub MAXI_awburst: [bool; 2],
  #[port(output)]
  pub MAXI_awvalid: bool,
  #[port(input)]
  pub MAXI_awready: bool,
  // axi w
  #[port(output)]
  pub MAXI_wdata: [bool; 64],
  #[port(output)]
  pub MAXI_wstrb: [bool; 8],
  #[port(output)]
  pub MAXI_wlast: bool,
  #[port(output)]
  pub MAXI_wvalid: bool,
  #[port(input)]
  pub MAXI_wready: bool,
  // axi b
  #[port(input)]
  pub MAXI_bid: [bool; 4],
  #[port(input)]
  pub MAXI_bresp: [bool; 2],
  #[port(input)]
  pub MAXI_bvalid: bool,
  #[port(output)]
  pub MAXI_bready: bool,
  // axi ar
  #[port(output)]
  pub MAXI_arid: [bool; 4],
  #[port(output)]
  pub MAXI_araddr: [bool; 32],
  #[port(output)]
  pub MAXI_arlen: [bool; 8],
  #[port(output)]
  pub MAXI_arsize: [bool; 3],
  #[port(output)]
  pub MAXI_arburst: [bool; 2],
  #[port(output)]
  pub MAXI_arvalid: bool,
  #[port(input)]
  pub MAXI_arready: bool,
  // axi r
  #[port(input)]
  pub MAXI_rid: [bool; 4],
  #[port(input)]
  pub MAXI_rdata: [bool; 64],
  #[port(input)]
  pub MAXI_rresp: [bool; 2],
  #[port(input)]
  pub MAXI_rlast: bool,
  #[port(input)]
  pub MAXI_rvalid: bool,
  #[port(output)]
  pub MAXI_rready: bool,
  // debug
  #[port(output)]
  pub debug_commit: bool,
  #[port(output)]
  pub debug_pc: [bool; 64],
  #[port(output)]
  pub debug_rf_wnum: [bool; 5],
  #[port(output)]
  pub debug_rf_wdata: [bool; 64],
  // debug csr
  #[port(output)]
  pub debug_csr_interrupt: bool,
  #[port(output)]
  pub debug_csr_mcycle: [bool; 64],
  #[port(output)]
  pub debug_csr_mip: [bool; 64],
  #[port(output)]
  pub debug_csr_minstret: [bool; 64],
  // perf
  #[port(output)]
  pub debug_perf_icache_req: bool,
  #[port(output)]
  pub debug_perf_icache_hit: bool,
  #[port(output)]
  pub debug_perf_dcache_req: bool,
  #[port(output)]
  pub debug_perf_dcache_hit: bool,
  #[port(output)]
  pub debug_perf_bru_pred_branch: bool,
  #[port(output)]
  pub debug_perf_bru_pred_fail: bool,
}
