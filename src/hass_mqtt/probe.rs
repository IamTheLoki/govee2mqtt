use crate::hass_mqtt::base::{Device, EntityConfig, Origin};
use crate::hass_mqtt::instance::EntityInstance;
use crate::hass_mqtt::sensor::{SensorConfig, StateClass};
use crate::hass_mqtt::switch::SwitchConfig;
use crate::probe_thermometer::ProbeState;
use crate::service::device::Device as ServiceDevice;
use crate::service::hass::{
    availability_topic, switch_instance_state_topic, topic_safe_id, topic_safe_string, HassClient,
};
use crate::service::state::StateHandle;
use crate::temperature::{TemperatureValue, DEVICE_CLASS_TEMPERATURE};
use async_trait::async_trait;

pub const LIVE_POLLING_INSTANCE: &str = "probeLivePolling";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProbeChannel {
    Core,
    Ambient,
    CoreMax,
    CoreMin,
    AmbientMax,
    AmbientMin,
}

impl ProbeChannel {
    pub const ALL: [ProbeChannel; 6] = [
        Self::Core,
        Self::Ambient,
        Self::CoreMax,
        Self::CoreMin,
        Self::AmbientMax,
        Self::AmbientMin,
    ];

    fn label(&self) -> &'static str {
        match self {
            Self::Core => "Core",
            Self::Ambient => "Ambient",
            Self::CoreMax => "Core Max",
            Self::CoreMin => "Core Min",
            Self::AmbientMax => "Ambient Max",
            Self::AmbientMin => "Ambient Min",
        }
    }

    fn is_limit(&self) -> bool {
        !matches!(self, Self::Core | Self::Ambient)
    }

    fn value(&self, state: &ProbeState) -> Option<f64> {
        match self {
            Self::Core => state.reading.core,
            Self::Ambient => state.reading.ambient,
            Self::CoreMax => state.limits?.core_max,
            Self::CoreMin => state.limits?.core_min,
            Self::AmbientMax => state.limits?.ambient_max,
            Self::AmbientMin => state.limits?.ambient_min,
        }
    }
}

pub struct ProbeTemperatureSensor {
    sensor: SensorConfig,
    device_id: String,
    state: StateHandle,
    probe: u8,
    channel: ProbeChannel,
}

impl ProbeTemperatureSensor {
    pub async fn new(
        device: &ServiceDevice,
        state: &StateHandle,
        probe: u8,
        channel: ProbeChannel,
    ) -> Self {
        let unique_id = format!(
            "sensor-{id}-probe{probe}-{channel}",
            id = topic_safe_id(device),
            channel = topic_safe_string(channel.label())
        );

        Self {
            sensor: SensorConfig {
                base: EntityConfig {
                    availability_topic: availability_topic(),
                    name: Some(format!("Probe {probe} {}", channel.label())),
                    entity_category: channel.is_limit().then(|| "diagnostic".to_string()),
                    origin: Origin::default(),
                    device: Device::for_device(device),
                    unique_id: unique_id.clone(),
                    device_class: Some(DEVICE_CLASS_TEMPERATURE),
                    icon: None,
                },
                state_topic: format!("gv2mqtt/sensor/{unique_id}/state"),
                state_class: Some(StateClass::Measurement),
                unit_of_measurement: Some(
                    state.get_temperature_scale().await.unit_of_measurement(),
                ),
                json_attributes_topic: None,
            },
            device_id: device.id.to_string(),
            state: state.clone(),
            probe,
            channel,
        }
    }
}

#[async_trait]
impl EntityInstance for ProbeTemperatureSensor {
    async fn publish_config(&self, state: &StateHandle, client: &HassClient) -> anyhow::Result<()> {
        self.sensor.publish(state, client).await
    }

    async fn notify_state(&self, client: &HassClient) -> anyhow::Result<()> {
        let device = self
            .state
            .device_by_id(&self.device_id)
            .await
            .expect("device to exist");

        let Some(probe_state) = device.probes.get(&self.probe) else {
            return Ok(());
        };
        if self.channel.is_limit() && probe_state.limits.is_none() {
            return Ok(());
        }
        let celsius = self.channel.value(probe_state);

        let value = match celsius {
            Some(c) => {
                let scale = self.state.get_temperature_scale().await;
                let value = TemperatureValue::with_celsius(c)
                    .as_unit(scale.into())
                    .value();
                format!("{value:.2}")
            }
            // An unplugged probe: this is the payload that makes
            // hass show the entity as "unknown"
            None => "None".to_string(),
        };
        self.sensor.notify_state(client, &value).await
    }
}

pub struct ProbeLivePollingSwitch {
    switch: SwitchConfig,
    device_id: String,
    state: StateHandle,
}

impl ProbeLivePollingSwitch {
    pub fn new(device: &ServiceDevice, state: &StateHandle) -> Self {
        let id = topic_safe_id(device);
        Self {
            switch: SwitchConfig {
                base: EntityConfig {
                    availability_topic: availability_topic(),
                    name: Some("Live Polling".to_string()),
                    device_class: None,
                    origin: Origin::default(),
                    device: Device::for_device(device),
                    unique_id: format!("gv2mqtt-{id}-{LIVE_POLLING_INSTANCE}"),
                    entity_category: Some("config".to_string()),
                    icon: Some("mdi:update".to_string()),
                },
                command_topic: format!("gv2mqtt/switch/{id}/command/{LIVE_POLLING_INSTANCE}"),
                state_topic: switch_instance_state_topic(device, LIVE_POLLING_INSTANCE),
            },
            device_id: device.id.to_string(),
            state: state.clone(),
        }
    }
}

#[async_trait]
impl EntityInstance for ProbeLivePollingSwitch {
    async fn publish_config(&self, state: &StateHandle, client: &HassClient) -> anyhow::Result<()> {
        self.switch.publish(state, client).await
    }

    async fn notify_state(&self, client: &HassClient) -> anyhow::Result<()> {
        let device = self
            .state
            .device_by_id(&self.device_id)
            .await
            .expect("device to exist");
        client
            .publish(
                &self.switch.state_topic,
                if device.probe_live_polling {
                    "ON"
                } else {
                    "OFF"
                },
            )
            .await
    }
}
