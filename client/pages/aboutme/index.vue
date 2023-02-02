<template>
<div>
<div class="term">

  <cat-command :name="this.$route.path"/>

 <div class="content">

  
  <nav-tab class="nav"/>

  <file-header :edited="edited" :name="this.$route.name" :created="20230124" />

  <div class="rawr">

      <div class="aboutme">
        <h2>~ me</h2>
        
      Hey, im berryes/berry, or at least thats what my online alias is. 
      <br> I'm a junior 'fullstack' developer student.
      I'm privacy and FOSS advocate who is interested mainly in tech and also music.
      <br>
      <br>
      let mut languages :Vec&ltstr&gt = vec!["Rust","JS","Python","C#"];<br>
      let me :Vec&ltstr&gt = vec!["Student","He/him","Paranoid","Linux user"];<br>



        <h2>~ setup</h2>
        <img src="/neofetch.png">
      </div>

  </div>
 
 </div>

</div>
<all-footer />

</div>


</template>

<script>

import CatCommand from '../../components/catCommand.vue'
import FileHeader from '../../components/fileHeader.vue'
import navTab from '../../components/navTab.vue'
import allFooter from '../../components/allFooter.vue'

export default {
  components: { navTab, FileHeader, CatCommand,allFooter },
  name: 'Blog',
  head: {
    title: 'blog - berryez.xyz'
  },

   data: ()=>{
    return{
      edited: '20230114',
      meleg: null,
    }
  },
  fetchOnServer: true,
 async fetch(){
   this.meleg = await fetch("http://127.0.0.1:8000/api").then((response) => response.text() )
 },


}

</script>

<style scoped>
.aboutme {
  font-size: 0.6em;
}
</style>