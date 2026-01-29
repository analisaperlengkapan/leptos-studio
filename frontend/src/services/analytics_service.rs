//! Analytics Service
//!
//! Provides application usage analytics, performance metrics,
//! and session tracking for debugging and optimization.

use gloo_net::http::Request;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Performance metric types
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum MetricType {
    /// Time to render (ms)
    RenderTime,
    /// Component count
    ComponentCount,
    /// Memory usage estimate
    MemoryUsage,
    /// Event processing time
    EventProcessingTime,
    /// User interaction latency
    InteractionLatency,
}

/// A single metric entry
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Metric {
    pub metric_type: MetricType,
    pub value: f64,
    pub timestamp: f64,
    pub labels: HashMap<String, String>,
}

impl Metric {
    pub fn new(metric_type: MetricType, value: f64) -> Self {
        Self {
            metric_type,
            value,
            timestamp: js_sys::Date::now(),
            labels: HashMap::new(),
        }
    }

    pub fn with_label(mut self, key: &str, value: &str) -> Self {
        self.labels.insert(key.to_string(), value.to_string());
        self
    }
}

/// Session information
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SessionInfo {
    pub session_id: String,
    pub start_time: f64,
    pub total_actions: u32,
    pub components_created: u32,
    pub components_deleted: u32,
    pub exports_count: u32,
    pub saves_count: u32,
    pub undo_count: u32,
    pub redo_count: u32,
}

impl SessionInfo {
    pub fn new() -> Self {
        Self {
            session_id: uuid::Uuid::new_v4().to_string(),
            start_time: js_sys::Date::now(),
            total_actions: 0,
            components_created: 0,
            components_deleted: 0,
            exports_count: 0,
            saves_count: 0,
            undo_count: 0,
            redo_count: 0,
        }
    }

    /// Get session duration in seconds
    pub fn duration_seconds(&self) -> f64 {
        (js_sys::Date::now() - self.start_time) / 1000.0
    }
}

impl Default for SessionInfo {
    fn default() -> Self {
        Self::new()
    }
}

/// Analytics service state
#[derive(Clone, Copy)]
pub struct AnalyticsService {
    /// Session information
    session: RwSignal<SessionInfo>,
    /// Recent metrics (limited buffer)
    metrics: RwSignal<Vec<Metric>>,
    /// Analytics enabled flag
    enabled: RwSignal<bool>,
    /// Last sync timestamp
    last_synced: RwSignal<f64>,
}

impl AnalyticsService {
    /// Maximum metrics to keep in memory
    const MAX_METRICS: usize = 1000;

    /// Create new analytics service
    pub fn new() -> Self {
        let service = Self {
            session: RwSignal::new(SessionInfo::new()),
            metrics: RwSignal::new(Vec::new()),
            enabled: RwSignal::new(true),
            last_synced: RwSignal::new(0.0),
        };
        service.start_auto_flush();
        service
    }

    /// Provide analytics in Leptos context
    pub fn provide_context() {
        provide_context(Self::new());
    }

    fn start_auto_flush(&self) {
        let service = *self;
        // Flush every 30 seconds
        leptos::task::spawn_local(async move {
            let mut interval = gloo_timers::future::IntervalStream::new(30_000);
            use futures::StreamExt;
            while interval.next().await.is_some() {
                if service.is_enabled() && !service.metrics.get().is_empty() {
                    if let Err(e) = service.flush_to_backend().await {
                        web_sys::console::warn_1(&format!("Analytics flush failed: {}", e).into());
                    }
                }
            }
        });
    }

    /// Use analytics from Leptos context
    pub fn use_context() -> Self {
        expect_context::<Self>()
    }

    /// Check if analytics is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled.get()
    }

    /// Enable/disable analytics
    pub fn set_enabled(&self, enabled: bool) {
        self.enabled.set(enabled);
    }

    /// Record a metric
    pub fn record(&self, metric: Metric) {
        if !self.is_enabled() {
            return;
        }

        self.metrics.update(|metrics| {
            metrics.push(metric);
            if metrics.len() > Self::MAX_METRICS {
                metrics.remove(0);
            }
        });
    }

    /// Record render time
    pub fn record_render_time(&self, duration_ms: f64) {
        self.record(Metric::new(MetricType::RenderTime, duration_ms));
    }

    /// Record component count
    pub fn record_component_count(&self, count: usize) {
        self.record(Metric::new(MetricType::ComponentCount, count as f64));
    }

    /// Track action
    pub fn track_action(&self, action: &str) {
        if !self.is_enabled() {
            return;
        }

        self.session.update(|s| {
            s.total_actions += 1;
            match action {
                "component_create" => s.components_created += 1,
                "component_delete" => s.components_deleted += 1,
                "export" => s.exports_count += 1,
                "save" => s.saves_count += 1,
                "undo" => s.undo_count += 1,
                "redo" => s.redo_count += 1,
                _ => {}
            }
        });
    }

    /// Get session info
    pub fn session_info(&self) -> SessionInfo {
        self.session.get()
    }

    pub fn last_synced(&self) -> f64 {
        self.last_synced.get()
    }

    fn get_api_base() -> String {
        let runtime_base = window()
            .get("LEPTOS_API_URL")
            .and_then(|val| val.as_string());

        let base = runtime_base
            .or_else(|| option_env!("API_URL").map(|s| s.to_string()))
            .unwrap_or_else(|| "http://localhost:3000".to_string());

        format!("{}/api/analytics", base.trim_end_matches('/'))
    }

    /// Flush metrics to backend
    pub async fn flush_to_backend(&self) -> Result<(), String> {
        let metrics = self.metrics.get();
        let session = self.session.get();

        // Transform metrics to backend expected format (AnalyticsData)
        // struct AnalyticsData { session_id, timestamp, event_type, payload }
        // We will send a batch of events.

        let events: Vec<serde_json::Value> = metrics
            .iter()
            .map(|m| {
                serde_json::json!({
                    "session_id": session.session_id,
                    "timestamp": m.timestamp,
                    "event_type": format!("{:?}", m.metric_type),
                    "payload": serde_json::to_value(m).unwrap_or_default(),
                })
            })
            .collect();

        // Also add session info update as an event
        let session_event = serde_json::json!({
            "session_id": session.session_id,
            "timestamp": js_sys::Date::now(),
            "event_type": "SessionUpdate",
            "payload": serde_json::to_value(&session).unwrap_or_default(),
        });

        let mut batch = events;
        batch.push(session_event);

        let body = serde_json::json!({
            "events": batch
        });

        let resp = Request::post(&Self::get_api_base())
            .json(&body)
            .map_err(|e| e.to_string())?
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if !resp.ok() {
            return Err(format!("Server returned {}", resp.status()));
        }

        self.last_synced.set(js_sys::Date::now());
        // Clear metrics after successful flush to avoid duplication on next sync
        self.clear_metrics();

        Ok(())
    }

    /// Get metrics summary
    pub fn metrics_summary(&self) -> MetricsSummary {
        let metrics = self.metrics.get();

        // Calculate render time stats
        let render_times: Vec<f64> = metrics
            .iter()
            .filter(|m| m.metric_type == MetricType::RenderTime)
            .map(|m| m.value)
            .collect();

        let avg_render_time = if render_times.is_empty() {
            0.0
        } else {
            render_times.iter().sum::<f64>() / render_times.len() as f64
        };

        let max_render_time = render_times.iter().cloned().fold(0.0, f64::max);
        let min_render_time = render_times.iter().cloned().fold(f64::INFINITY, f64::min);

        MetricsSummary {
            total_metrics: metrics.len(),
            avg_render_time,
            max_render_time,
            min_render_time: if min_render_time == f64::INFINITY {
                0.0
            } else {
                min_render_time
            },
            session: self.session_info(),
        }
    }

    /// Clear all metrics
    pub fn clear_metrics(&self) {
        self.metrics.set(Vec::new());
    }

    /// Get recent metrics
    pub fn recent_metrics(&self, count: usize) -> Vec<Metric> {
        let metrics = self.metrics.get();
        metrics.into_iter().rev().take(count).collect()
    }

    /// Export metrics as JSON
    pub fn export_json(&self) -> Result<String, serde_json::Error> {
        let data = AnalyticsExport {
            session: self.session_info(),
            metrics: self.metrics.get(),
            summary: self.metrics_summary(),
        };
        serde_json::to_string_pretty(&data)
    }
}

impl Default for AnalyticsService {
    fn default() -> Self {
        Self::new()
    }
}

/// Summary of metrics
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MetricsSummary {
    pub total_metrics: usize,
    pub avg_render_time: f64,
    pub max_render_time: f64,
    pub min_render_time: f64,
    pub session: SessionInfo,
}

/// Export format for analytics
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AnalyticsExport {
    pub session: SessionInfo,
    pub metrics: Vec<Metric>,
    pub summary: MetricsSummary,
}

// Unit tests (WASM-only because they use js_sys::Date::now())
#[cfg(all(test, target_arch = "wasm32"))]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_session_creation() {
        let session = SessionInfo::new();
        assert!(!session.session_id.is_empty());
        assert_eq!(session.total_actions, 0);
    }

    #[wasm_bindgen_test]
    fn test_metric_creation() {
        let metric = Metric::new(MetricType::RenderTime, 16.5).with_label("component", "Canvas");

        assert_eq!(metric.metric_type, MetricType::RenderTime);
        assert_eq!(metric.value, 16.5);
        assert_eq!(metric.labels.get("component"), Some(&"Canvas".to_string()));
    }

    #[wasm_bindgen_test]
    fn test_metrics_summary() {
        let analytics = AnalyticsService::new();
        analytics.record_render_time(10.0);
        analytics.record_render_time(20.0);
        analytics.record_render_time(30.0);

        let summary = analytics.metrics_summary();
        assert_eq!(summary.total_metrics, 3);
        assert!((summary.avg_render_time - 20.0).abs() < 0.001);
        assert_eq!(summary.max_render_time, 30.0);
        assert_eq!(summary.min_render_time, 10.0);
    }

    #[wasm_bindgen_test]
    fn test_action_tracking() {
        let analytics = AnalyticsService::new();
        analytics.track_action("component_create");
        analytics.track_action("component_create");
        analytics.track_action("undo");

        let session = analytics.session_info();
        assert_eq!(session.total_actions, 3);
        assert_eq!(session.components_created, 2);
        assert_eq!(session.undo_count, 1);
    }
}
