//use crate::metrics::model::MetricsType;
use lazy_static::lazy_static;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum MetricsKey {
    //app
    SysTotalMemory,
    AppRssMemory,
    AppVmsMemory,
    AppMemoryUsage,
    AppCpuUsage,
    //config
    ConfigDataSize,
    ConfigListenerClientSize,
    ConfigListenerKeySize,
    ConfigSubscriberListenerKeySize,
    ConfigSubscriberListenerValueSize,
    ConfigSubscriberClientSize,
    ConfigSubscriberClientValueSize,
    ConfigIndexTenantSize,
    ConfigIndexConfigSize,
    //naming
    NamingServiceSize,
    NamingInstanceSize,
    NamingSubscriberListenerKeySize,
    NamingSubscriberListenerValueSize,
    NamingSubscriberClientSize,
    NamingSubscriberClientValueSize,
    NamingEmptyServiceSetSize,
    NamingEmptyServiceSetItemSize,
    NamingInstanceMetaSetSize,
    NamingInstanceMetaSetItemSize,
    NamingHealthyTimeoutSetSize,
    NamingHealthyTimeoutSetItemSize,
    NamingUnhealthyTimeoutSetSize,
    NamingUnhealthyTimeoutSetItemSize,
    NamingClientInstanceSetKeySize,
    NamingClientInstanceSetValueSize,
    NamingIndexTenantSize,
    NamingIndexGroupSize,
    NamingIndexServiceSize,
    //grpc
    GrpcConnSize,
    GrpcConnActiveTimeoutSetItemSize,
    GrpcConnResponseTimeoutSetItemSize,
    //grpc request
    GrpcRequestHandleRtHistogram,
    GrpcRequestTotalCount,
    //http api request
    HttpRequestHandleRtHistogram,
    HttpRequestTotalCount,
}

lazy_static! {
    /// 用于有序遍历打印信息
    pub static ref ORDER_ALL_KEYS: Vec<MetricsKey> = vec![
        //app
        /*
        MetricsKey::SysTotalMemory,
        MetricsKey::AppRssMemory,
        MetricsKey::AppVmsMemory,
        MetricsKey::AppMemoryUsage,
        MetricsKey::AppCpuUsage,
         */
        //config
        MetricsKey::ConfigDataSize,
        MetricsKey::ConfigListenerClientSize,
        MetricsKey::ConfigListenerKeySize,
        MetricsKey::ConfigSubscriberListenerKeySize,
        MetricsKey::ConfigSubscriberListenerValueSize,
        MetricsKey::ConfigSubscriberClientSize,
        MetricsKey::ConfigSubscriberClientValueSize,
        MetricsKey::ConfigIndexTenantSize,
        MetricsKey::ConfigIndexConfigSize,
        //naming
        MetricsKey::NamingServiceSize,
        MetricsKey::NamingInstanceSize,
        MetricsKey::NamingSubscriberListenerKeySize,
        MetricsKey::NamingSubscriberListenerValueSize,
        MetricsKey::NamingSubscriberClientSize,
        MetricsKey::NamingSubscriberClientValueSize,
        MetricsKey::NamingEmptyServiceSetSize,
        MetricsKey::NamingEmptyServiceSetItemSize,
        MetricsKey::NamingInstanceMetaSetSize,
        MetricsKey::NamingInstanceMetaSetItemSize,
        MetricsKey::NamingHealthyTimeoutSetSize,
        MetricsKey::NamingHealthyTimeoutSetItemSize,
        MetricsKey::NamingUnhealthyTimeoutSetSize,
        MetricsKey::NamingUnhealthyTimeoutSetItemSize,
        MetricsKey::NamingClientInstanceSetKeySize,
        MetricsKey::NamingClientInstanceSetValueSize,
        MetricsKey::NamingIndexTenantSize,
        MetricsKey::NamingIndexGroupSize,
        MetricsKey::NamingIndexServiceSize,
        //grpc
        MetricsKey::GrpcConnSize,
        MetricsKey::GrpcConnActiveTimeoutSetItemSize,
        MetricsKey::GrpcConnResponseTimeoutSetItemSize,
        //grpc request
        MetricsKey::GrpcRequestHandleRtHistogram,
        MetricsKey::GrpcRequestTotalCount,
        //http request
        MetricsKey::HttpRequestHandleRtHistogram,
        MetricsKey::HttpRequestTotalCount,
    ];
}

impl MetricsKey {
    pub fn get_key(&self) -> &'static str {
        match &self {
            MetricsKey::SysTotalMemory => "sys_total_memory",
            MetricsKey::AppRssMemory => "app_rss_memory",
            MetricsKey::AppVmsMemory => "app_vms_memory",
            MetricsKey::AppMemoryUsage => "app_memory_usage",
            MetricsKey::AppCpuUsage => "app_cpu_usage",
            MetricsKey::ConfigDataSize => "ConfigDataSize",
            MetricsKey::ConfigListenerClientSize => "ConfigListenerClientSize",
            MetricsKey::ConfigListenerKeySize => "ConfigListenerKeySize",
            MetricsKey::ConfigSubscriberListenerKeySize => "ConfigSubscriberListenerKeySize",
            MetricsKey::ConfigSubscriberListenerValueSize => "ConfigSubscriberListenerValueSize",
            MetricsKey::ConfigSubscriberClientSize => "ConfigSubscriberClientSize",
            MetricsKey::ConfigSubscriberClientValueSize => "ConfigSubscriberClientValueSize",
            MetricsKey::ConfigIndexTenantSize => "ConfigIndexTenantSize",
            MetricsKey::ConfigIndexConfigSize => "ConfigIndexConfigSize",
            MetricsKey::NamingServiceSize => "NamingServiceSize",
            MetricsKey::NamingInstanceSize => "NamingInstanceSize",
            MetricsKey::NamingSubscriberListenerKeySize => "NamingSubscriberListenerKeySize",
            MetricsKey::NamingSubscriberListenerValueSize => "NamingSubscriberListenerValueSize",
            MetricsKey::NamingSubscriberClientSize => "NamingSubscriberClientSize",
            MetricsKey::NamingSubscriberClientValueSize => "NamingSubscriberClientValueSize",
            MetricsKey::NamingEmptyServiceSetSize => "NamingEmptyServiceSetSize",
            MetricsKey::NamingEmptyServiceSetItemSize => "NamingEmptyServiceSetItemSize",
            MetricsKey::NamingInstanceMetaSetSize => "NamingInstanceMetaSetSize",
            MetricsKey::NamingInstanceMetaSetItemSize => "NamingInstanceMetaSetItemSize",
            MetricsKey::NamingHealthyTimeoutSetSize => "NamingHealthyTimeoutSetSize",
            MetricsKey::NamingHealthyTimeoutSetItemSize => "NamingHealthyTimeoutSetItemSize",
            MetricsKey::NamingUnhealthyTimeoutSetSize => "NamingUnhealthyTimeoutSetSize",
            MetricsKey::NamingUnhealthyTimeoutSetItemSize => "NamingUnhealthyTimeoutSetItemSize",
            MetricsKey::NamingClientInstanceSetKeySize => "NamingClientInstanceSetKeySize",
            MetricsKey::NamingClientInstanceSetValueSize => "NamingClientInstanceSetValueSize",
            MetricsKey::NamingIndexTenantSize => "NamingIndexTenantSize",
            MetricsKey::NamingIndexGroupSize => "NamingIndexGroupSize",
            MetricsKey::NamingIndexServiceSize => "NamingIndexServiceSize",
            MetricsKey::GrpcConnSize => "GrpcConnSize",
            MetricsKey::GrpcConnActiveTimeoutSetItemSize => "GrpcConnActiveTimeoutSetItemSize",
            MetricsKey::GrpcConnResponseTimeoutSetItemSize => "GrpcConnResponseTimeoutSetItemSize",
            //grpc request
            MetricsKey::GrpcRequestHandleRtHistogram => "GrpcRequestHandleRtHistogram",
            MetricsKey::GrpcRequestTotalCount => "GrpcRequestTotalCount",
            //http request
            MetricsKey::HttpRequestHandleRtHistogram => "HttpRequestHandleRtHistogram",
            MetricsKey::HttpRequestTotalCount => "HttpRequestTotalCount",
        }
    }

    /*
    pub fn get_metrics_type(&self) -> MetricsType {
        match &self {
            MetricsKey::ConfigDataSize => MetricsType::Gauge,
            MetricsKey::ConfigListenerClientSize => MetricsType::Gauge,
            MetricsKey::ConfigListenerKeySize => MetricsType::Gauge,
            MetricsKey::ConfigSubscriberListenerKeySize => MetricsType::Gauge,
            MetricsKey::ConfigSubscriberListenerValueSize => MetricsType::Gauge,
            MetricsKey::ConfigSubscriberClientSize => MetricsType::Gauge,
            MetricsKey::ConfigSubscriberClientValueSize => MetricsType::Gauge,
            MetricsKey::ConfigIndexTenantSize => MetricsType::Gauge,
            MetricsKey::ConfigIndexConfigSize => MetricsType::Gauge,
            MetricsKey::NamingServiceSize => MetricsType::Gauge,
            MetricsKey::NamingInstanceSize => MetricsType::Gauge,
            MetricsKey::NamingSubscriberListenerKeySize => MetricsType::Gauge,
            MetricsKey::NamingSubscriberListenerValueSize => MetricsType::Gauge,
            MetricsKey::NamingSubscriberClientSize => MetricsType::Gauge,
            MetricsKey::NamingSubscriberClientValueSize => MetricsType::Gauge,
            MetricsKey::NamingEmptyServiceSetSize => MetricsType::Gauge,
            MetricsKey::NamingEmptyServiceSetItemSize => MetricsType::Gauge,
            MetricsKey::NamingInstanceMetaSetSize => MetricsType::Gauge,
            MetricsKey::NamingInstanceMetaSetItemSize => MetricsType::Gauge,
            MetricsKey::NamingHealthyTimeoutSetSize => MetricsType::Gauge,
            MetricsKey::NamingHealthyTimeoutSetItemSize => MetricsType::Gauge,
            MetricsKey::NamingUnhealthyTimeoutSetSize => MetricsType::Gauge,
            MetricsKey::NamingUnhealthyTimeoutSetItemSize => MetricsType::Gauge,
            MetricsKey::NamingClientInstanceSetKeySize => MetricsType::Gauge,
            MetricsKey::NamingClientInstanceSetValueSize => MetricsType::Gauge,
            MetricsKey::NamingIndexTenantSize => MetricsType::Gauge,
            MetricsKey::NamingIndexGroupSize => MetricsType::Gauge,
            MetricsKey::NamingIndexServiceSize => MetricsType::Gauge,
            MetricsKey::GrpcConnSize => MetricsType::Gauge,
            MetricsKey::GrpcConnActiveTimeoutSetItemSize => MetricsType::Gauge,
            MetricsKey::GrpcConnResponseTimeoutSetItemSize => MetricsType::Gauge,
        }
    }
     */
}
