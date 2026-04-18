// Bookstore Books Service
//! Circuit Breaker for Recommendation Service.
// Author: Karan Manoj Shah <kmshah2@cs.cmu.edu>

use std::{
  sync::{LazyLock, Mutex},
  time::{Duration, Instant},
};

/// State of the circuit breaker.
#[derive(Default)]
struct CircuitBreakerState {
  /// Timestamp when the circuit last transitioned to open.
  opened_at: Option<Instant>,
}

/// Decision returned by circuit evaluation before an outbound request.
pub enum CircuitDecision {
  /// Request is allowed; probe indicates first attempt after the open window.
  Allow { is_probe_after_open_window: bool },
  /// Request should be rejected immediately while circuit is open.
  Reject,
}

pub const CIRCUIT_OPEN_WINDOW: Duration = Duration::from_secs(60);
static RECOMMENDATION_CIRCUIT_BREAKER: LazyLock<Mutex<CircuitBreakerState>> =
  LazyLock::new(|| Mutex::new(CircuitBreakerState::default()));

/// Evaluate whether the recommendation call should proceed right now.
pub fn check_circuit() -> CircuitDecision {
  let state = RECOMMENDATION_CIRCUIT_BREAKER
    .lock()
    .expect("circuit lock poisoned");

  match state.opened_at {
    Some(opened_at) if opened_at.elapsed() < CIRCUIT_OPEN_WINDOW => CircuitDecision::Reject,
    Some(_) => CircuitDecision::Allow {
      is_probe_after_open_window: true,
    },
    None => CircuitDecision::Allow {
      is_probe_after_open_window: false,
    },
  }
}

/// Open the circuit due to a timeout from the recommendation service.
pub fn open_circuit() {
  let mut state = RECOMMENDATION_CIRCUIT_BREAKER
    .lock()
    .expect("circuit lock poisoned");
  state.opened_at = Some(Instant::now());
}

/// Close the circuit after a successful recommendation round trip.
pub fn close_circuit() {
  let mut state = RECOMMENDATION_CIRCUIT_BREAKER
    .lock()
    .expect("circuit lock poisoned");
  state.opened_at = None;
}
