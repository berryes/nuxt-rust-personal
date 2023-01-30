<template>
<div>
<div class="term">

  <cat-command :name="this.$route.path"/>

 <div class="content">

  
  <nav-tab class="nav"/>

  <file-header :edited="edited" :name="this.$route.name" :created="20230124" />

  <div class="rawr">
      Hey, im berryes/berry, or at least thats what my alias is. I'm a junior 'fullstack' developer.
      I'm a privacy and FOSS advocate who is interested mainly in tech.

      
      <div class="subpages">
      <p>subpages:</p>
        <ul>
          <li><router-link to="/aboutme/music"> fav music rn</router-link></li>
        </ul>
      </div>

      <br>
      <div class="aboutme">
        <h3>~ me</h3>
      let mut languages :Vec&ltstr&gt = vec!["Rust","JS","Python","C#"];<br>
      let me :Vec&ltstr&gt = vec!["Student","He/him","Paranoid","Linux user"];<br>


        <h3>~ setup</h3>
        <ul>
        <li>PC: Rog Strix G512LI</li>
        <li>Distro: Fedora 37 Workstation</li>
        <li>RAM: 16 GB</li>
        <li>GPU: NVIDIA GeForce GTX 1650 Ti (4GB)</li>
        <li>CPU: Intel Core i5-10300H @ 2.50GHz</li>
        <li>Display: Samsung Odyssey G5 32"</li>
        </ul>
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

 async fetch(){
    await axios({
      method:"GET",
      url: "https://localhost:8000/"
    }).then(()=>{
      this.meleg = re
    })
 },
 fetchOnServer:true

}

</script>

<style scoped>
.aboutme {
  font-size: 0.6em;
}
</style>