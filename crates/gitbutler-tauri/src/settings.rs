#![allow(deprecated)]
use but_api::commands::settings;
use but_settings::api::{
    ClaudeUpdate, FeatureFlagsUpdate, FetchUpdate, ReviewsUpdate, TelemetryUpdate,
};
use but_settings::AppSettingsWithDiskSync;
use tauri::State;
use tracing::instrument;

use but_api::error::Error;

#[tauri::command(async)]
#[instrument(skip(app_settings_sync), err(Debug))]
pub fn update_onboarding_complete(
    app_settings_sync: State<'_, AppSettingsWithDiskSync>,
    update: bool,
) -> Result<(), Error> {
    settings::update_onboarding_complete(
        &app_settings_sync,
        settings::UpdateOnboardingCompleteParams { update },
    )
}

#[tauri::command(async)]
#[instrument(skip(app_settings_sync), err(Debug))]
pub fn update_telemetry(
    app_settings_sync: State<'_, AppSettingsWithDiskSync>,
    update: TelemetryUpdate,
) -> Result<(), Error> {
    settings::update_telemetry(
        &app_settings_sync,
        settings::UpdateTelemetryParams { update },
    )
}

#[tauri::command(async)]
#[instrument(skip(app_settings_sync), err(Debug))]
pub fn update_telemetry_distinct_id(
    app_settings_sync: State<'_, AppSettingsWithDiskSync>,
    app_distinct_id: Option<String>,
) -> Result<(), Error> {
    settings::update_telemetry_distinct_id(
        &app_settings_sync,
        settings::UpdateTelemetryDistinctIdParams { app_distinct_id },
    )
}

#[tauri::command(async)]
#[instrument(skip(app_settings_sync), err(Debug))]
pub fn update_feature_flags(
    app_settings_sync: State<'_, AppSettingsWithDiskSync>,
    update: FeatureFlagsUpdate,
) -> Result<(), Error> {
    settings::update_feature_flags(
        &app_settings_sync,
        settings::UpdateFeatureFlagsParams { update },
    )
}

#[tauri::command(async)]
#[instrument(skip(app_settings_sync), err(Debug))]
pub fn update_claude(
    app_settings_sync: State<'_, AppSettingsWithDiskSync>,
    update: ClaudeUpdate,
) -> Result<(), Error> {
    settings::update_claude(&app_settings_sync, settings::UpdateClaudeParams { update })
}

#[tauri::command(async)]
#[instrument(skip(app_settings_sync), err(Debug))]
pub fn update_fetch(
    app_settings_sync: State<'_, AppSettingsWithDiskSync>,
    update: FetchUpdate,
) -> Result<(), Error> {
    settings::update_fetch(&app_settings_sync, settings::UpdateFetchParams { update })
}

#[tauri::command(async)]
#[instrument(skip(app_settings_sync), err(Debug))]
pub fn update_reviews(
    app_settings_sync: State<'_, AppSettingsWithDiskSync>,
    update: ReviewsUpdate,
) -> Result<(), Error> {
    settings::update_reviews(&app_settings_sync, settings::UpdateReviewsParams { update })
}
