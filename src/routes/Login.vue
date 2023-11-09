<script setup lang="ts">
import { onMounted, ref } from 'vue';
import CommandInvoker from '../invoker'
import { useRouter } from 'vue-router';

const router = useRouter();

const inputCredential = ref({
    phone: '',
    password: ''
})

const mannualLogin = ref(true);

const saveCredential = ref(false);

onMounted(() => {
    CommandInvoker.getCredential().then((res)=>{
        if(res) {
            CommandInvoker.login(res).then((_)=>{
                router.push({name:"main"})
            }).catch((err)=>{
                console.log(err);
                mannualLogin.value = true;
            })
        } else {
            mannualLogin.value = true;
        }
    }).catch((err)=>{
        console.log(err);
        mannualLogin.value = true;
    })
})

function login() {
    CommandInvoker.login(inputCredential.value).then((_) => {
        console.log("success");
        if(saveCredential) {
            console.log("saving");
            CommandInvoker.saveCredential(inputCredential.value).then((_)=>{
                console.log("saved");
            }).catch((err)=>{
                console.log(err);
            })
        }
        router.push({name:"main"})
    }).catch((err) => {
        console.log(err);
    })
}

const notNullRules = [
    (v: string) => !!v || 'Required'
]

</script>

<template>
    <div class="content-center h-screen" v-if="mannualLogin">
        <div class="flex flex-col content-center items-center justify-center h-full">
            <v-form class="w-96 flex flex-col items-stretch" :model="inputCredential">
                <v-text-field clearable variant="outlined" label="手机号码" v-model="inputCredential.phone" placeholder="Phone" type="text" :rules="notNullRules"/>
                <v-text-field clearable variant="outlined" label="密码" v-model="inputCredential.password" placeholder="Password" type="password" :rules="notNullRules"/>
                <v-checkbox class="self-end" v-model="saveCredential" label="自动登录"/>
                <v-btn class="w-full" color="primary" type="button" @click="login">登录</v-btn>
            </v-form>
        </div>
    </div>
</template>