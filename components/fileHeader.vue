<template>
    <div class="file">
    <br>
    {{name}}.html | created: {{created}} | edited: {{edited}} 
    <hr>
  </div>
</template>

<script>
export default {
    props: ['name','edited','created']
}
</script>

<style>

</style>