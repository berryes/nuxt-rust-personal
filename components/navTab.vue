<template>
  <div>
    <ul>
        <router-link to="/"><ol> home</ol> </router-link>
        <router-link to="/aboutme"><ol> aboutme</ol> </router-link>
        <router-link to="/privacy"><ol> privacy</ol> </router-link>
    </ul>

  </div>

</template>

<script>
export default {

}
</script>

<style scoped>

ul {
   display:flex; 
}
ol:hover {
  background-color: rgb(41, 211, 41);
}
ol:hover{
   color: white;
}
</style>