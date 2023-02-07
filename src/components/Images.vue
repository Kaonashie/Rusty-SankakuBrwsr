<script setup lang="ts">
import {ref} from 'vue'

const imageSampleApi = "http://localhost:8080/api/image-sample"
const imageFetched = ref(false)
const imageObject = ref()

async function fetchImages() {
    const res = await fetch("http://localhost:8080/api/image-info")
    const data = await res.json()
    imageObject.value = data
    imageFetched.value = true
    console.log(data)
}
</script>

<template>
    <button type="button" @click="fetchImages">Display Images</button>
    <div class="grid grid-cols-3 gap-4 px-16">

        <div class="w-[200px] h-[300px]"
             v-if="imageFetched" v-for="image in imageObject.cached_images">
            <img :src="imageSampleApi + '/' + image.post_id" class="w-[100%] h-[100%] object-cover "/>
        </div>
    </div>
</template>