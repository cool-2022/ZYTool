<template>
    <div class="tool-page">
        <div class="container">
            <!-- 页面标题 -->
            <div class="page-header">
                <a-typography-title :level="2">地图定位工具</a-typography-title>
                <a-typography-paragraph>获取当前位置并在地图上显示</a-typography-paragraph>
            </div>

            <!-- 控制面板 -->
            <a-card title="位置信息" :bordered="false" style="margin-bottom: 24px;">
                <a-space direction="vertical" style="width: 100%" :size="16">
                    <a-row :gutter="16">
                        <a-col :xs="24" :md="12">
                            <a-button type="primary" @click="getCurrentLocation" :loading="loading" size="large" block>
                                <environment-outlined /> 获取当前位置
                            </a-button>
                        </a-col>
                        <a-col :xs="24" :md="12">
                            <a-button @click="showManualInput = !showManualInput" size="large" block>
                                <edit-outlined /> {{ showManualInput ? '隐藏' : '显示' }}手动输入
                            </a-button>
                        </a-col>
                    </a-row>

                    <!-- 手动输入区域 -->
                    <a-card v-if="showManualInput" size="small" title="手动输入位置" :bordered="false" style="background: var(--bg-secondary);">
                        <a-tabs v-model:activeKey="inputMode">
                            <!-- 地址搜索 -->
                            <a-tab-pane key="address" tab="地址搜索">
                                <a-space direction="vertical" style="width: 100%;" :size="12">
                                    <a-input-search
                                        v-model:value="searchAddress"
                                        placeholder="输入地址，例如：北京市天安门"
                                        size="large"
                                        enter-button="搜索"
                                        :loading="searching"
                                        @search="searchByAddress"
                                    >
                                        <template #prefix>
                                            <search-outlined />
                                        </template>
                                    </a-input-search>
                                    <a-alert type="info" message="支持输入省市区、街道、门牌号、POI名称等" show-icon />
                                </a-space>
                            </a-tab-pane>

                            <!-- 经纬度输入 -->
                            <a-tab-pane key="coordinates" tab="经纬度输入">
                                <a-space direction="vertical" style="width: 100%;" :size="12">
                                    <a-row :gutter="12">
                                        <a-col :span="12">
                                            <a-input
                                                v-model:value="manualLongitude"
                                                placeholder="经度 (如: 116.397428)"
                                                size="large"
                                            >
                                                <template #prefix>
                                                    经度:
                                                </template>
                                            </a-input>
                                        </a-col>
                                        <a-col :span="12">
                                            <a-input
                                                v-model:value="manualLatitude"
                                                placeholder="纬度 (如: 39.90923)"
                                                size="large"
                                            >
                                                <template #prefix>
                                                    纬度:
                                                </template>
                                            </a-input>
                                        </a-col>
                                    </a-row>
                                    <a-button type="primary" @click="locateByCoordinates" :loading="searching" block size="large">
                                        <aim-outlined /> 定位到此坐标
                                    </a-button>
                                    <a-alert type="info" message="经度范围: -180 ~ 180, 纬度范围: -90 ~ 90" show-icon />
                                </a-space>
                            </a-tab-pane>
                        </a-tabs>
                    </a-card>

                    <a-divider style="margin: 12px 0;" />

                    <!-- 位置信息显示 -->
                    <div v-if="locationInfo" class="location-info">
                        <a-descriptions :column="1" bordered size="small">
                            <a-descriptions-item label="经度">
                                <span class="coordinate-value">{{ locationInfo.longitude }}</span>
                                <a-button size="small" type="link" @click="copyToClipboard(locationInfo.longitude)">
                                    复制
                                </a-button>
                            </a-descriptions-item>
                            <a-descriptions-item label="纬度">
                                <span class="coordinate-value">{{ locationInfo.latitude }}</span>
                                <a-button size="small" type="link" @click="copyToClipboard(locationInfo.latitude)">
                                    复制
                                </a-button>
                            </a-descriptions-item>
                            <a-descriptions-item label="精度">
                                {{ locationInfo.accuracy }} 米
                            </a-descriptions-item>
                            <a-descriptions-item label="获取时间">
                                {{ locationInfo.timestamp }}
                            </a-descriptions-item>
                            <a-descriptions-item label="地址信息" v-if="locationInfo.address">
                                {{ locationInfo.address }}
                            </a-descriptions-item>
                        </a-descriptions>
                    </div>

                    <a-alert v-else type="info" message="点击上方按钮获取当前位置" show-icon />
                </a-space>
            </a-card>

            <!-- 地图显示区域 -->
            <a-card title="地图显示" :bordered="false">
                <div id="map-container" class="map-container"></div>
                <a-alert v-if="!mapLoaded" type="warning" message="地图加载中..." show-icon style="margin-top: 16px;" />
            </a-card>

            <!-- 使用说明 -->
            <a-divider>使用说明</a-divider>
            <a-card :bordered="false">
                <a-descriptions :column="1">
                    <a-descriptions-item label="功能说明">
                        使用浏览器的地理位置API获取当前位置，支持地址搜索和手动输入经纬度定位
                    </a-descriptions-item>
                    <a-descriptions-item label="地址搜索">
                        支持输入省市区、街道、门牌号、POI名称等进行地址搜索定位
                    </a-descriptions-item>
                    <a-descriptions-item label="经纬度输入">
                        可以手动输入经纬度坐标进行精确定位，经度范围: -180~180, 纬度范围: -90~90
                    </a-descriptions-item>
                    <a-descriptions-item label="权限要求">
                        需要授予浏览器访问位置信息的权限，首次使用时浏览器会弹出授权提示
                    </a-descriptions-item>
                    <a-descriptions-item label="精度说明">
                        定位精度取决于设备和网络环境，通常在10-100米之间
                    </a-descriptions-item>
                    <a-descriptions-item label="注意事项">
                        HTTPS环境下定位更准确，部分浏览器在HTTP环境下可能无法使用定位功能
                    </a-descriptions-item>
                </a-descriptions>
            </a-card>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { message } from 'ant-design-vue'
import { EnvironmentOutlined, EditOutlined, SearchOutlined, AimOutlined } from '@ant-design/icons-vue'

interface LocationInfo {
    longitude: string
    latitude: string
    accuracy: number
    timestamp: string
    address?: string
}

const loading = ref(false)
const searching = ref(false)
const locationInfo = ref<LocationInfo | null>(null)
const mapLoaded = ref(false)
const showManualInput = ref(false)
const inputMode = ref('address')
const searchAddress = ref('')
const manualLongitude = ref('')
const manualLatitude = ref('')
let map: any = null
let marker: any = null
let circle: any = null
let AMap: any = null

// 加载高德地图API
const loadAMapScript = (): Promise<void> => {
    return new Promise((resolve, reject) => {
        if ((window as any).AMap) {
            AMap = (window as any).AMap
            resolve()
            return
        }

        const script = document.createElement('script')
        // 使用你的高德地图API Key，并加载插件
        script.src = 'https://webapi.amap.com/maps?v=2.0&key=82aaaef6e38ad9523d993e795b2fd05c&plugin=AMap.Geocoder'
        script.async = true
        script.onload = () => {
            AMap = (window as any).AMap
            console.log('高德地图API加载成功', AMap)
            resolve()
        }
        script.onerror = (error) => {
            console.error('高德地图API加载失败', error)
            reject(error)
        }
        document.head.appendChild(script)
    })
}

// 初始化地图
const initMap = async () => {
    try {
        console.log('开始加载高德地图...')
        await loadAMapScript()
        
        console.log('开始初始化地图容器...')
        map = new AMap.Map('map-container', {
            zoom: 15,
            center: [121.451825, 31.407630], // 默认中心点（宝信软件）
            viewMode: '2D', // 改为2D模式，更稳定
            resizeEnable: true
        })

        // 等待地图加载完成
        map.on('complete', () => {
            mapLoaded.value = true
            console.log('地图加载完成')
            message.success('地图加载成功')
        })

        // 监听地图加载错误
        map.on('error', (error: any) => {
            console.error('地图加载错误:', error)
            message.error('地图加载出错')
        })

    } catch (error) {
        console.error('地图初始化失败:', error)
        message.error('地图加载失败，请检查网络连接或API Key配置')
    }
}

// 获取当前位置
const getCurrentLocation = () => {
    if (!navigator.geolocation) {
        message.error('您的浏览器不支持地理定位功能')
        return
    }

    if (!mapLoaded.value) {
        message.warning('地图还在加载中，请稍后再试')
        return
    }

    loading.value = true

    navigator.geolocation.getCurrentPosition(
        async (position) => {
            const { longitude, latitude, accuracy } = position.coords
            
            console.log('获取到位置:', { longitude, latitude, accuracy })
            
            updateLocationInfo(longitude, latitude, accuracy, '浏览器定位')
            await updateMapMarker(longitude, latitude, accuracy)

            loading.value = false
            message.success('定位成功')
        },
        (error) => {
            loading.value = false
            
            let errorMessage = '定位失败'
            switch (error.code) {
                case error.PERMISSION_DENIED:
                    errorMessage = '用户拒绝了位置权限请求'
                    break
                case error.POSITION_UNAVAILABLE:
                    errorMessage = '位置信息不可用'
                    break
                case error.TIMEOUT:
                    errorMessage = '定位请求超时'
                    break
            }
            
            message.error(errorMessage)
            console.error('定位错误:', error)
        },
        {
            enableHighAccuracy: true, // 启用高精度
            timeout: 10000, // 超时时间10秒
            maximumAge: 0 // 不使用缓存
        }
    )
}

// 通过地址搜索定位
const searchByAddress = async () => {
    if (!searchAddress.value.trim()) {
        message.warning('请输入要搜索的地址')
        return
    }

    if (!AMap) {
        message.error('地图未加载完成')
        return
    }

    if (!mapLoaded.value) {
        message.warning('地图还在加载中，请稍后再试')
        return
    }

    searching.value = true
    console.log('开始搜索地址:', searchAddress.value)

    try {
        // 确保Geocoder可用
        if (!AMap.Geocoder) {
            console.error('Geocoder插件未加载')
            message.error('地址搜索功能未加载，请刷新页面重试')
            searching.value = false
            return
        }

        const geocoder = new AMap.Geocoder({
            city: '全国' // 设置城市范围为全国
        })
        
        console.log('Geocoder创建成功，开始查询...')
        
        geocoder.getLocation(searchAddress.value, (status: string, result: any) => {
            console.log('地址搜索结果:', { status, result })
            searching.value = false
            
            if (status === 'complete' && result.info === 'OK') {
                if (result.geocodes && result.geocodes.length > 0) {
                    const location = result.geocodes[0].location
                    const longitude = location.lng
                    const latitude = location.lat
                    const address = result.geocodes[0].formattedAddress
                    
                    console.log('解析到坐标:', { longitude, latitude, address })
                    
                    updateLocationInfo(longitude, latitude, 50, '地址搜索', address)
                    updateMapMarker(longitude, latitude, 50)
                    
                    message.success('地址搜索成功')
                } else {
                    console.error('搜索结果为空')
                    message.error('未找到该地址，请尝试更详细的地址信息')
                }
            } else {
                console.error('地址搜索失败:', status, result)
                message.error(`地址搜索失败: ${result?.info || status}`)
            }
        })
    } catch (error) {
        searching.value = false
        console.error('地址搜索异常:', error)
        message.error('地址搜索失败: ' + (error as Error).message)
    }
}

// 通过经纬度定位
const locateByCoordinates = async () => {
    const lng = parseFloat(manualLongitude.value)
    const lat = parseFloat(manualLatitude.value)

    // 验证经纬度
    if (isNaN(lng) || isNaN(lat)) {
        message.error('请输入有效的经纬度数值')
        return
    }

    if (lng < -180 || lng > 180) {
        message.error('经度范围应在 -180 到 180 之间')
        return
    }

    if (lat < -90 || lat > 90) {
        message.error('纬度范围应在 -90 到 90 之间')
        return
    }

    searching.value = true

    try {
        updateLocationInfo(lng, lat, 50, '手动输入')
        await updateMapMarker(lng, lat, 50)
        
        searching.value = false
        message.success('定位成功')
    } catch (error) {
        searching.value = false
        message.error('定位失败')
        console.error('定位错误:', error)
    }
}

// 更新位置信息
const updateLocationInfo = (longitude: number, latitude: number, accuracy: number, source: string, address?: string) => {
    console.log('更新位置信息:', { longitude, latitude, accuracy, source, address })
    
    locationInfo.value = {
        longitude: longitude.toFixed(6),
        latitude: latitude.toFixed(6),
        accuracy: Math.round(accuracy),
        timestamp: new Date().toLocaleString('zh-CN'),
        address: address
    }

    // 如果没有提供地址，尝试逆地理编码获取
    if (!address && AMap && mapLoaded.value) {
        console.log('开始逆地理编码...')
        try {
            // 确保Geocoder可用
            if (!AMap.Geocoder) {
                console.error('Geocoder插件未加载')
                return
            }

            const geocoder = new AMap.Geocoder({
                radius: 1000, // 范围1000米
                extensions: 'all' // 返回详细信息
            })
            
            geocoder.getAddress([longitude, latitude], (status: string, result: any) => {
                console.log('逆地理编码结果:', { status, result })
                if (status === 'complete' && result.info === 'OK') {
                    if (locationInfo.value && result.regeocode) {
                        locationInfo.value.address = result.regeocode.formattedAddress
                        console.log('地址获取成功:', result.regeocode.formattedAddress)
                    }
                } else {
                    console.error('逆地理编码失败:', status, result)
                }
            })
        } catch (error) {
            console.error('逆地理编码异常:', error)
        }
    }
}

// 更新地图标记
const updateMapMarker = async (longitude: number, latitude: number, accuracy: number) => {
    console.log('开始更新地图标记:', { longitude, latitude, accuracy, map, AMap, mapLoaded: mapLoaded.value })
    
    if (!map || !AMap) {
        console.error('地图或AMap未初始化')
        message.warning('地图未加载完成，无法显示标记')
        return
    }

    if (!mapLoaded.value) {
        console.error('地图未加载完成')
        message.warning('地图未加载完成，请稍后再试')
        return
    }

    try {
        // 设置地图中心点
        console.log('设置地图中心点...')
        map.setCenter([longitude, latitude])
        map.setZoom(15)
        
        // 移除旧标记和圆圈
        if (marker) {
            console.log('移除旧标记')
            map.remove(marker)
            marker = null
        }
        if (circle) {
            console.log('移除旧圆圈')
            map.remove(circle)
            circle = null
        }
        
        // 添加新标记 - 使用简化的方式
        console.log('添加新标记...')
        marker = new AMap.Marker({
            position: new AMap.LngLat(longitude, latitude),
            title: '定位位置'
        })
        
        map.add(marker)
        console.log('标记添加成功')

        // 添加圆形表示精度范围
        console.log('添加精度圆圈...')
        circle = new AMap.Circle({
            center: new AMap.LngLat(longitude, latitude),
            radius: accuracy,
            fillColor: '#1890ff',
            fillOpacity: 0.2,
            strokeColor: '#1890ff',
            strokeWeight: 2
        })
        
        map.add(circle)
        console.log('圆圈添加成功')
        
        // 调整视野以包含圆圈
        map.setFitView([marker, circle])
        
    } catch (error) {
        console.error('更新地图标记失败:', error)
        message.error('地图标记更新失败: ' + (error as Error).message)
    }
}

// 复制到剪贴板
const copyToClipboard = (text: string) => {
    navigator.clipboard.writeText(text)
        .then(() => message.success('复制成功'))
        .catch(() => message.error('复制失败'))
}

// 组件挂载时初始化地图
onMounted(() => {
    initMap()
})

// 组件卸载时清理
onUnmounted(() => {
    if (map) {
        map.destroy()
    }
})
</script>

<style scoped>
.tool-page {
    min-height: 100vh;
    background: var(--gradient-bg);
    padding: 2rem 0;
}

.container {
    max-width: 1400px;
    margin: 0 auto;
    padding: 0 20px;
}

.page-header {
    text-align: center;
    margin-bottom: 2rem;
    animation: fadeInDown 0.6s ease-out;
}

.page-header :deep(.ant-typography) {
    color: var(--text-primary);
}

@keyframes fadeInDown {
    from {
        opacity: 0;
        transform: translateY(-20px);
    }

    to {
        opacity: 1;
        transform: translateY(0);
    }
}

.tool-page :deep(.ant-card) {
    border-radius: var(--border-radius-lg);
    box-shadow: var(--shadow-md);
    transition: all var(--transition-speed);
    border: 1px solid var(--border-color);
    background: var(--bg-primary);
}

.tool-page :deep(.ant-card-head) {
    background: var(--gradient-primary);
    color: white;
    border-radius: var(--border-radius-lg) var(--border-radius-lg) 0 0;
}

.tool-page :deep(.ant-card-head-title) {
    color: white;
    font-weight: 600;
}

.tool-page :deep(.ant-btn-primary) {
    background: var(--gradient-primary);
    border: none;
}

.location-info {
    background: var(--bg-secondary);
    padding: 1rem;
    border-radius: var(--border-radius);
}

.coordinate-value {
    font-family: 'Courier New', monospace;
    font-weight: 600;
    font-size: 16px;
    color: var(--primary-color);
    margin-right: 8px;
}

.map-container {
    width: 100%;
    height: 500px;
    border-radius: var(--border-radius);
    overflow: hidden;
    background: #f0f0f0;
}

.tool-page :deep(.ant-descriptions-item-label) {
    color: var(--text-primary);
    background: var(--bg-secondary);
    font-weight: 600;
}

.tool-page :deep(.ant-descriptions-item-content) {
    color: var(--text-primary);
    background: var(--bg-primary);
}

.tool-page :deep(.ant-divider-inner-text) {
    color: var(--text-primary);
}

.tool-page :deep(.ant-alert) {
    border-radius: var(--border-radius);
}

.tool-page :deep(.ant-tabs) {
    color: var(--text-primary);
}

.tool-page :deep(.ant-tabs-tab) {
    color: var(--text-secondary);
}

.tool-page :deep(.ant-tabs-tab-active) {
    color: var(--primary-color);
}

.tool-page :deep(.ant-input) {
    background: var(--bg-primary);
    border-color: var(--border-color);
    color: var(--text-primary);
}

.tool-page :deep(.ant-input::placeholder) {
    color: var(--text-secondary);
}

.tool-page :deep(.ant-input-prefix) {
    color: var(--text-secondary);
}

.tool-page :deep(.ant-input-search-button) {
    background: var(--gradient-primary);
    border: none;
}
</style>
