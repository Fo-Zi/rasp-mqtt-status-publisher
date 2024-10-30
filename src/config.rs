pub struct MqttConfig{
    pub publish_to: &'static str,
}

pub struct AppConfig{
    pub mqtt_config: MqttConfig,
    pub id: &'static str,
}

pub const MQTT_CONFIG: MqttConfig = MqttConfig {
    publish_to: "",
};

pub const APP_CONFIG: AppConfig = AppConfig {
    mqtt_config: MQTT_CONFIG,
    id: "RASP_DEV_TEST",
};