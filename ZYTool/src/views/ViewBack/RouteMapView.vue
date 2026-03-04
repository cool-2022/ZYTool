<template>
    <div class="tool-page">
        <div class="container">
            <!-- 页面标题 -->
            <div class="page-header">
                <a-typography-title :level="2">地图路径规划</a-typography-title>
                <a-typography-paragraph>输入起点和终点获取导航路线</a-typography-paragraph>
            </div>

            <!-- 控制面板 -->
            <a-card title="路径规划" :bordered="false" style="margin-bottom: 24px;">
                <a-space direction="vertical" style="width: 100%" :size="16">
                    <a-row :gutter="16">
                        <a-col :xs="24" :md="12">
                            <a-form layout="vertical">
                                <a-form-item label="起点">
                                    <a-input-group compact>
                                        <a-input v-model:value="startAddress" placeholder="请输入起点地址"
                                            style="width: calc(100% - 32px)" />
                                        <a-button @click="getCurrentLocation('start')" type="default">
                                            <environment-outlined />
                                        </a-button>
                                    </a-input-group>
                                </a-form-item>

                                <a-form-item label="终点">
                                    <a-input v-model:value="endAddress" placeholder="请输入终点地址"
                                        @pressEnter="searchRoute" />
                                </a-form-item>

                                <a-form-item>
                                    <a-button type="primary" @click="searchRoute" :loading="searching" size="large"
                                        block>
                                        <search-outlined /> 搜索路线
                                    </a-button>
                                </a-form-item>
                            </a-form>
                        </a-col>

                        <a-col :xs="24" :md="12">
                            <a-card title="路线信息" :bordered="false" size="small"
                                style="background: var(--bg-secondary);">
                                <div v-if="routeInfo" class="route-info">
                                    <a-descriptions :column="1" bordered size="small">
                                        <a-descriptions-item label="总距离">
                                            {{ formatDistance(routeInfo.distance) }}
                                        </a-descriptions-item>
                                        <a-descriptions-item label="预计耗时">
                                            {{ formatDuration(routeInfo.duration) }}
                                        </a-descriptions-item>
                                        <a-descriptions-item label="起点坐标">
                                            {{ routeInfo.startLocation.lat.toFixed(6) }}, {{
                                                routeInfo.startLocation.lng.toFixed(6) }}
                                        </a-descriptions-item>
                                        <a-descriptions-item label="终点坐标">
                                            {{ routeInfo.endLocation.lat.toFixed(6) }}, {{
                                                routeInfo.endLocation.lng.toFixed(6) }}
                                        </a-descriptions-item>
                                    </a-descriptions>

                                    <a-button type="primary" @click="clearRoute" size="small" style="margin-top: 16px;">
                                        清除路线
                                    </a-button>
                                </div>
                                <a-alert v-else type="info" message="请输入起点和终点地址" show-icon />
                            </a-card>
                        </a-col>
                    </a-row>
                </a-space>
            </a-card>

            <!-- 地图显示区域 -->
            <a-card title="路线地图" :bordered="false">
                <div id="route-map-container" class="map-container"></div>
                <a-alert v-if="!mapLoaded" type="warning" message="地图加载中..." show-icon style="margin-top: 16px;" />
            </a-card>

            <!-- 路线详情 -->
            <a-card v-if="routeInfo && routeInfo.steps" title="路线详情" :bordered="false" style="margin-top: 24px;">
                <a-steps :current="-1" :items="routeInfo.steps.map((step, index) => ({
                    title: step.instruction,
                    description: `${formatDistance(step.distance)} | ${formatDuration(step.duration)}`
                }))" direction="vertical" size="small" />
            </a-card>

            <!-- 使用说明 -->
            <a-divider style="margin-top: 32px;">使用说明</a-divider>
            <a-card :bordered="false">
                <a-descriptions :column="1">
                    <a-descriptions-item label="功能说明">
                        输入起点和终点地址，获取导航路线并在地图上显示
                    </a-descriptions-item>
                    <a-descriptions-item label="起点输入">
                        支持手动输入地址或使用当前位置作为起点
                    </a-descriptions-item>
                    <a-descriptions-item label="终点输入">
                        输入目标地址，系统将计算最优路线
                    </a-descriptions-item>
                    <a-descriptions-item label="路线显示">
                        在地图上显示路线轨迹及关键节点
                    </a-descriptions-item>
                    <a-descriptions-item label="路线详情">
                        提供详细路线指引和距离时间信息
                    </a-descriptions-item>
                </a-descriptions>
            </a-card>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { message } from 'ant-design-vue'
import { EnvironmentOutlined, SearchOutlined } from '@ant-design/icons-vue'
import { ApiService } from '../../services/api'

interface RoutePoint {
    lat: number
    lng: number
    name?: string
}

interface RouteStep {
    instruction: string
    distance: number
    duration: number
    start_location: RoutePoint
    end_location: RoutePoint
}

interface RouteInfo {
    distance: number
    duration: number
    steps: RouteStep[]
    polyline: RoutePoint[]
    startLocation: RoutePoint
    endLocation: RoutePoint
}

const searching = ref(false)
const mapLoaded = ref(false)
const startAddress = ref('')
const endAddress = ref('')
const routeInfo = ref<RouteInfo | null>(null)

let map: any = null
let AMap: any = null
let routePolyline: any = null
let startMarker: any = null
let endMarker: any = null

// 加载高德地图API
const loadAMapScript = (): Promise<void> => {
    return new Promise((resolve, reject) => {
        if ((window as any).AMap) {
            AMap = (window as any).AMap
            resolve()
            return
        }

        const script = document.createElement('script')
        script.src = 'https://webapi.amap.com/maps?v=2.0&key=82aaaef6e38ad9523d993e795b2fd05c&plugin=AMap.Driving,AMap.Autocomplete,AMap.PlaceSearch'
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
        map = new AMap.Map('route-map-container', {
            zoom: 12,
            center: [121.451825, 31.407630], // 默认中心点（宝信软件）
            viewMode: '2D',
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

// 格式化距离
const formatDistance = (distance: number): string => {
    if (distance < 1000) {
        return `${Math.round(distance)} 米`
    }
    return `${(distance / 1000).toFixed(2)} 公里`
}

// 格式化持续时间
const formatDuration = (duration: number): string => {
    const minutes = Math.round(duration / 60)
    if (minutes < 60) {
        return `${minutes} 分钟`
    }
    const hours = Math.floor(minutes / 60)
    const remainingMinutes = minutes % 60
    return `${hours} 小时 ${remainingMinutes} 分钟`
}

// 获取当前位置
const getCurrentLocation = (target: 'start' | 'end') => {
    if (!navigator.geolocation) {
        message.error('您的浏览器不支持地理定位功能')
        return
    }

    if (!mapLoaded.value) {
        message.warning('地图还在加载中，请稍后再试')
        return
    }

    navigator.geolocation.getCurrentPosition(
        async (position) => {
            const { latitude, longitude } = position.coords

            if (target === 'start') {
                startAddress.value = `我的位置 (${latitude.toFixed(6)}, ${longitude.toFixed(6)})`
            } else {
                endAddress.value = `我的位置 (${latitude.toFixed(6)}, ${longitude.toFixed(6)})`
            }

            message.success('位置获取成功')
        },
        (error) => {
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
            enableHighAccuracy: true,
            timeout: 10000,
            maximumAge: 0
        }
    )
}

// 搜索路线
const searchRoute = async () => {
    if (!startAddress.value.trim()) {
        message.warning('请输入起点地址')
        return
    }

    if (!endAddress.value.trim()) {
        message.warning('请输入终点地址')
        return
    }

    if (!mapLoaded.value) {
        message.warning('地图还在加载中，请稍后再试')
        return
    }

    searching.value = true

    try {
        // 先通过地址获取坐标
        const startCoords = await geocodeAddress(startAddress.value)

        const endCoords = await geocodeAddress(endAddress.value)

        if (!startCoords || !endCoords) {
            message.error('无法解析地址，请检查输入')
            return
        }

        // 调用后端API获取路线
        const response = await ApiService.getRoute({
            origin: {
                lat: startCoords.lat,
                lng: startCoords.lng
            },
            destination: {
                lat: endCoords.lat,
                lng: endCoords.lng
            }
        })
        // const response = await api.post('/tools/map/route', {
        //     origin: {
        //         lat: startCoords.lat,
        //         lng: startCoords.lng
        //     },
        //     destination: {
        //         lat: endCoords.lat,
        //         lng: endCoords.lng
        //     }
        // })

        routeInfo.value = {
            ...response.data,
            startLocation: startCoords,
            endLocation: endCoords
        }

        // 在地图上绘制路线
        drawRouteOnMap(startCoords, endCoords, response.data.polyline)

        message.success('路线规划成功')
    } catch (error) {
        console.error('路线规划失败:', error)
        message.error('路线规划失败，请稍后重试')
    } finally {
        searching.value = false
    }
}

// 地址解析
const geocodeAddress = async (address: string): Promise<RoutePoint | null> => {

    console.log('开始解析地址:', address);
    if (!AMap) {
        console.error('地图API未加载')
        return null
    }

    return new Promise((resolve, reject) => {
        const geocoder = new AMap.Geocoder({
            city: '全国'
        })

        geocoder.getLocation(address, (status: string, result: any) => {
            if (status === 'complete' && result.info === 'OK') {
                if (result.geocodes && result.geocodes.length > 0) {
                    const location = result.geocodes[0].location
                    resolve({
                        lat: location.lat,
                        lng: location.lng
                    })
                } else {
                    resolve(null)
                }
            } else {
                console.error('地址解析失败:', status, result)
                resolve(null)
            }
        })
    })
}

// 在地图上绘制路线
const drawRouteOnMap = (start: RoutePoint, end: RoutePoint, polyline: RoutePoint[]) => {
    // 清除之前的路线和标记
    clearPreviousMarkers()

    // 添加起点标记
    startMarker = new AMap.Marker({
        position: new AMap.LngLat(start.lng, start.lat),
        icon: 'https://webapi.amap.com/theme/v1.3/markers/n/mark_b.png',
        title: '起点'
    })
    map.add(startMarker)

    // 添加终点标记
    endMarker = new AMap.Marker({
        position: new AMap.LngLat(end.lng, end.lat),
        icon: 'https://webapi.amap.com/theme/v1.3/markers/n/mark_a.png',
        title: '终点'
    })
    map.add(endMarker)

    // 绘制路线
    if (polyline && polyline.length > 1) {
        const path = polyline.map(point => new AMap.LngLat(point.lng, point.lat))

        routePolyline = new AMap.Polyline({
            path: path,
            strokeColor: '#3366FF', // 线条颜色
            strokeWeight: 6, // 线条宽度
            strokeOpacity: 0.8 // 线条透明度
        })

        map.add(routePolyline)

        // 调整地图视野以适应路线
        map.setFitView([startMarker, endMarker, routePolyline])
    } else {
        // 如果没有polyline数据，使用简单的直线
        routePolyline = new AMap.Polyline({
            path: [new AMap.LngLat(start.lng, start.lat), new AMap.LngLat(end.lng, end.lat)],
            strokeColor: '#3366FF',
            strokeWeight: 6,
            strokeOpacity: 0.8
        })

        map.add(routePolyline)
        map.setFitView([startMarker, endMarker])
    }
}

// 清除路线
const clearRoute = () => {
    clearPreviousMarkers()
    routeInfo.value = null
    message.success('路线已清除')
}

// 清除之前的标记
const clearPreviousMarkers = () => {
    if (map && routePolyline) {
        map.remove(routePolyline)
        routePolyline = null
    }
    if (map && startMarker) {
        map.remove(startMarker)
        startMarker = null
    }
    if (map && endMarker) {
        map.remove(endMarker)
        endMarker = null
    }
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
    clearPreviousMarkers()
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

.route-info {
    background: var(--bg-secondary);
    padding: 1rem;
    border-radius: var(--border-radius);
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

.tool-page :deep(.ant-input) {
    background: var(--bg-primary);
    border-color: var(--border-color);
    color: var(--text-primary);
}

.tool-page :deep(.ant-input::placeholder) {
    color: var(--text-secondary);
}

.tool-page :deep(.ant-input-affix-wrapper) {
    background: var(--bg-primary);
    border-color: var(--border-color);
    color: var(--text-primary);
}

.tool-page :deep(.ant-steps-item-description) {
    color: var(--text-secondary);
}

.tool-page :deep(.ant-steps-item-title) {
    color: var(--text-primary);
}
</style>