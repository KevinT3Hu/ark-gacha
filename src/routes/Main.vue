<script setup lang="ts">
import { onMounted, reactive } from 'vue';
import CommandInvoker from '../invoker';
import { Gacha, PoolStatistics, TotalStatistics } from '../types';
import { ref } from 'vue';
import { Pie, Bar } from 'vue-chartjs';
import { computed } from 'vue';
import { Chart as ChartJS, ChartData, ArcElement, Legend, Tooltip, CategoryScale, LinearScale, BarElement } from 'chart.js';
import { watch } from 'vue';

ChartJS.register(ArcElement, Legend, Tooltip, CategoryScale, LinearScale, BarElement)

const allGacha = reactive<Gacha[]>([]);

interface PoolOption {
    title: string;
    value: string | undefined;
}

const currentPool = ref<PoolOption>({
    title: "全部",
    value: undefined
});

onMounted(() => {
    console.log("mounted");
    CommandInvoker.getAllGacha().then((res) => {
        allGacha.splice(0, allGacha.length, ...res);
        CommandInvoker.calculateStatistics(allGacha,currentPool.value.value).then((res) => {
            statistics.value = res;
        })
    })
})

const statistics = ref<TotalStatistics|PoolStatistics|undefined>(undefined);

const totalRolls = computed(()=>{
    if(statistics.value==undefined) return 0;
    return statistics.value.total;
})

const isTotal = computed(()=>{
    return currentPool.value.value==undefined;
})

const poolOptions = computed(()=>{
    if(statistics.value==undefined) return undefined;

    var options:{title:string,value:string|undefined}[] = statistics.value.allPools.map((pool)=>{
        return {
            title:pool,
            value:pool
        }
    })
    options.push({
        title:"全部卡池",
        value:undefined
    });
    return options;
})

const colors = [
    // 3 star
    "#5c5c5c",
    // 4 star
    "#00a8ff",
    // 5 star
    "#fbc531",
    // 6 star
    "#e84118"
]

const pieData = computed(() => {
    if(statistics.value==undefined) return undefined;

    var dataValues:number[] = [];
    var labels:string[] = [];

    statistics.value.starsCount.forEach((count,index)=>{
        labels.push((index+3)+" Star");
        dataValues.push(count);
    })

    const data:ChartData<"pie",number[],unknown> = {
        labels: labels,
        datasets: [
            {
                label: ' ',
                data: dataValues,
                backgroundColor: colors,
                hoverBorderWidth: 7,
            }
        ]
    };

    return data;
})

const waterPlaces = computed(()=>{
    if(statistics.value==undefined || !isTotal.value) return undefined;

    var data:{pool:string,wp:number}[] = [];

    const stat = statistics.value as TotalStatistics;
    stat.allPools.forEach((pool,index)=>{
        data.push({
            pool:pool,
            wp:stat.waterPlace[index]
        })
    })
    return data;
})

const poolRollsData = computed(() => {
    if(statistics.value?.allPools==undefined) return undefined;

    const stat = statistics.value as TotalStatistics;

    const data:ChartData<"bar",number[],unknown> = {
        labels: stat.allPools,
        datasets: [
            {
                indexAxis: 'y',
                barThickness: 8,
                label: 'Total Rolls',
                data: stat.poolsCount,
                backgroundColor: colors,
            }
        ],
    };

    return data;
})

watch(currentPool,(newVal)=>{
    CommandInvoker.calculateStatistics(allGacha,newVal.value).then((res) => {
        statistics.value = res;
    })
})

</script>

<template>
    <div>
        <!--topbar-->
        <div class="flex flex-row justify-between items-center bg-gray-800 text-white h-16">
            <div class="flex flex-row items-center">
                <div class="ml-4">
                    <h1 class="text-2xl font-bold">Arknights Gacha Visualizer</h1>
                </div>
            </div>
            <div class="flex flex-row items-center">
                <p class="mr-4">User Name</p>
            </div>
        </div>

        <div class="flex flex-row items-center justify-center">
            <v-select v-model="currentPool" :items="poolOptions" item-title="title" item-value="value" label="选择卡池" class="ml-2 mr-2 mt-2 max-w-xs" return-object></v-select>
            <div class=" w-5"/>
            <p class="text-xl font-bold">总抽数： {{totalRolls}}</p>
            <div class=" w-5"/>
            <p v-if="!isTotal" class="text-xl font-bold">水位： {{ (statistics as PoolStatistics).waterPlace }}</p>
        </div>

        <div class="flex flex-row flex-wrap justify-center" v-if="statistics!=undefined">
            <v-card class="m-3 card">
                <v-card-title class="mt-2">
                    <h1 class="text-2xl font-bold">抽取分布</h1>
                </v-card-title>
                <v-card-text>
                    <Pie responsive :data="pieData!!"></Pie>
                </v-card-text>
            </v-card>

            <v-card class="m-3 card" v-if="isTotal">
                <v-card-title class="mt-2">
                    <h1 class="text-2xl font-bold">卡池抽数</h1>
                </v-card-title>
                <v-card-text>
                    <Bar :data="poolRollsData!!"></Bar>
                </v-card-text>
            </v-card>

            <v-card class="m-3 card" v-if="isTotal">
                <v-card-title class="mt-2">
                    <h1 class="text-2xl font-bold">保底情况
                        <v-tooltip activator="parent" location="top">
                            由于十连的影响，水位会有小于等于10的误差
                        </v-tooltip>
                    </h1>
                </v-card-title>
                <v-card-text>
                    <div class="" v-for="wp in waterPlaces!!">
                        <p>{{wp.pool}}: {{wp.wp}}</p>
                    </div>
                </v-card-text>
            </v-card>
        </div>
    </div>
</template>

<style scoped>

.card {
    min-width: 300px;
}

</style>